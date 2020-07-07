use crate::edes::rdes::RDES;
use crate::edes::tdes::TDES;
use crate::edes_old::*;
use crate::sysctl;
use byteorder::ByteOrder;
use core::convert::TryInto;
use smoltcp::phy::{self, ChecksumCapabilities, Device, DeviceCapabilities};
use smoltcp::time::Instant;
use tm4c129x::EMAC0;

pub struct EphyReg(pub u8);

impl EphyReg {
    pub fn get(&self, emac0: &mut tm4c129x::EMAC0, phy_addr: u8) -> u16 {
        let reg_addr = self.0;

        assert!(phy_addr < 32);
        assert!(reg_addr < 32);

        assert!(emac0.miiaddr.read().miib().bit_is_clear());

        unsafe {
            emac0.miiaddr.modify(|_, w| {
                w.pla().bits(phy_addr);
                w.mii().bits(reg_addr);

                w.miiw().clear_bit();
                w.miib().set_bit();

                w
            });
        }

        while emac0.miiaddr.read().miib().bit_is_set() {}

        emac0.miidata.read().data().bits()
    }

    pub fn set(&self, emac0: &mut tm4c129x::EMAC0, phy_addr: u8, value: u16) {
        let reg_addr = self.0;

        assert!(phy_addr < 32);
        assert!(reg_addr < 32);

        assert!(emac0.miiaddr.read().miib().bit_is_clear());

        unsafe {
            emac0.miidata.write(|w| w.bits(value.into()));

            emac0.miiaddr.modify(|_, w| {
                w.pla().bits(phy_addr);
                w.mii().bits(reg_addr);

                w.miiw().set_bit();
                w.miib().set_bit();

                w
            });
        }

        while emac0.miiaddr.read().miib().bit_is_set() {}
    }
}

const ETHERNET_MTU: usize = 1500;
const NUM_TX_DESCRIPTORS: usize = 2;
const NUM_RX_DESCRIPTORS: usize = 10;

static RX_DESCRIPTORS: [RDES; NUM_RX_DESCRIPTORS] = [
    RDES::new(),
    RDES::new(),
    RDES::new(),
    RDES::new(),
    RDES::new(),
    RDES::new(),
    RDES::new(),
    RDES::new(),
    RDES::new(),
    RDES::new(),
];
static TX_DESCRIPTORS: [TDES; NUM_TX_DESCRIPTORS] = [TDES::new(), TDES::new()];

static mut RX_BUFFERS: [[u8; ETHERNET_MTU]; NUM_RX_DESCRIPTORS] =
    [[0; ETHERNET_MTU]; NUM_RX_DESCRIPTORS];
static mut TX_BUFFERS: [[u8; ETHERNET_MTU]; NUM_TX_DESCRIPTORS] =
    [[0; ETHERNET_MTU]; NUM_TX_DESCRIPTORS];

pub struct EthernetDevice {
    emac0: EMAC0,
    next_rx_descriptor: &'static RDES,
    next_tx_descriptor: &'static TDES,
}

impl EthernetDevice {
    pub fn new(
        lock: &sysctl::PowerControl,
        clocks: sysctl::Clocks,
        nvic: &mut cortex_m::peripheral::NVIC,
        mut emac0: EMAC0,
        ephy: crate::ephy::EPHY,
    ) -> EthernetDevice {
        sysctl::control_power(
            lock,
            sysctl::Domain::Emac0,
            sysctl::RunMode::Run,
            sysctl::PowerState::On,
        );
        sysctl::control_power(
            lock,
            sysctl::Domain::Ephy0,
            sysctl::RunMode::Run,
            sysctl::PowerState::On,
        );

        emac0.dmabusmod.modify(|_, w| w.swr().set_bit());

        while emac0.dmabusmod.read().swr().bit_is_set() {}

        emac0.pc.modify(|_, w| {
            // EMAC_PHY_TYPE_INTERNAL
            w.phyext().clear_bit();
            w.pintfs().imii();
            // EMAC_PHY_INT_MDIX_EN
            w.mdixen().set_bit();

            // EMAC_PHY_AN_100B_T_FULL_DUPLEX
            w.anen().set_bit();
            w.anmode()._100fd();

            w
        });

        let pc = emac0.pc.read();
        if pc.phyext().bit_is_clear() {
            sysctl::reset(lock, sysctl::Domain::Ephy0);
            for _ in 0..10000 {
                cortex_m::asm::nop();
            }
        }

        // TI's register definitions seem to disagree with the datasheet here - this
        // register should be RW, and also doesn't seem to have the CLKEN field we need.
        // For now just assert that the bit is already set to the value we expect.
        if pc.pintfs().is_rmii() {
            // emac0.cc.modify(|_, w| w.clken().set_bit());
            assert!(emac0.cc.read().bits() & 0x00010000 == 0x00010000);
        } else {
            // emac0.cc.modify(|_, w| w.clken().clear_bit());
            assert!(emac0.cc.read().bits() & 0x00010000 == 0);
        }

        sysctl::reset(lock, sysctl::Domain::Emac0);

        for _ in 0..1000 {
            cortex_m::asm::nop();
        }

        // Make sure that the DMA software reset is clear before continuing.
        while emac0.dmabusmod.read().swr().bit_is_set() {}

        emac0.dmabusmod.reset();

        unsafe {
            emac0.miiaddr.modify(|_, w| {
                w.cr().bits(if clocks.sysclk.0 < 20_000_000 {
                    panic!()
                } else if clocks.sysclk.0 < 35_000_000 {
                    0x8
                } else if clocks.sysclk.0 < 60_000_000 {
                    0xc
                } else if clocks.sysclk.0 < 100_000_000 {
                    0x0
                } else if clocks.sysclk.0 < 150_000_000 {
                    0x4
                } else {
                    panic!()
                })
            });
        }

        // Disable all the MMC interrupts as these are enabled by default at reset.
        unsafe {
            emac0.mmcrxim.write(|w| w.bits(0xffffffff));
            emac0.mmctxim.write(|w| w.bits(0xffffffff));
        }

        emac0.cfg.modify(|_, w| {
            // w.saddr().bits(0x02);
            w.cst().set_bit();
            w.ifg()._96();
            w.dupm().set_bit();
            w.fes().set_bit();
            w.ipc().set_bit();
            w.acs().set_bit();
            w.bl()._1024();

            w
        });

        emac0.wdogto.reset();

        emac0.dmaopmode.write(|w| {
            w.rsf().set_bit();
            w.tsf().set_bit();
            w.ttc()._64();
            w.rtc()._64();

            w
        });

        unsafe {
            for i in 0..NUM_TX_DESCRIPTORS {
                TX_DESCRIPTORS[i].tdes0.write(|w| {
                    w.bits(
                        DES0_TX_CTRL_LAST_SEG
                            | DES0_TX_CTRL_FIRST_SEG
                            | DES0_TX_CTRL_CHAINED
                            | DES0_TX_CTRL_IP_ALL_CKHSUMS,
                    )
                });
                TX_DESCRIPTORS[i]
                    .tdes1
                    .write(|w| w.bits(DES1_TX_CTRL_SADDR_INSERT));
                TX_DESCRIPTORS[i]
                    .tdes2
                    .write(|w| w.bits(&mut TX_BUFFERS[i] as *mut _ as *mut _ as u32));
                TX_DESCRIPTORS[i].tdes3.write(|w| {
                    w.bits(if i == NUM_TX_DESCRIPTORS - 1 {
                        &TX_DESCRIPTORS[0]
                    } else {
                        &TX_DESCRIPTORS[i + 1]
                    } as *const _ as u32)
                });
            }

            for i in 0..NUM_RX_DESCRIPTORS {
                RX_DESCRIPTORS[i].rdes0.write(|w| w.own().set_bit());
                RX_DESCRIPTORS[i].rdes1.write(|w| {
                    w.bits(
                        DES1_RX_CTRL_CHAINED | ((ETHERNET_MTU as u32) << DES1_RX_CTRL_BUFF1_SIZE_S),
                    )
                });
                RX_DESCRIPTORS[i]
                    .rdes2
                    .write(|w| w.bits(&mut RX_BUFFERS[i][0] as *mut u8 as u32));
                RX_DESCRIPTORS[i].rdes3.write(|w| {
                    w.bits(if i == (NUM_RX_DESCRIPTORS - 1) {
                        &RX_DESCRIPTORS[0]
                    } else {
                        &RX_DESCRIPTORS[i + 1]
                    } as *const _ as u32)
                });
            }

            emac0
                .rxdladdr
                .write(|w| w.bits(&RX_DESCRIPTORS as *const _ as u32));
            emac0
                .txdladdr
                .write(|w| w.bits(&TX_DESCRIPTORS as *const _ as u32));
        }

        {
            unsafe {
                let mac_addr = [0x00u8, 0x1A, 0xB6, 0x00, 0x02, 0x74];

                emac0.addr0h.write(|w| {
                    w.addrhi()
                        .bits(byteorder::LittleEndian::read_u16(&mac_addr[4..]))
                });
                emac0.addr0l.write(|w| {
                    w.addrlo()
                        .bits(byteorder::LittleEndian::read_u32(&mac_addr[..4]))
                });
            }
        }

        while ephy.bmsr.read(&mut emac0).linkstat().bit_is_clear() {}

        emac0.framefltr.modify(|_, w| {
            w.ra().set_bit();
            w.pr().set_bit();

            w
        });

        unsafe {
            emac0.dmaim.write(|w| w.bits(0xffff_ffff));
            emac0.ephyim.write(|w| w.bits(0xffff_ffff));
        }

        emac0.dmaopmode.modify(|_, w| {
            w.sr().set_bit();
            w.st().set_bit();

            w
        });
        emac0.cfg.modify(|_, w| {
            w.re().set_bit();
            w.te().set_bit();

            w
        });

        nvic.enable(tm4c129x::Interrupt::EMAC0);

        EthernetDevice {
            emac0,
            next_rx_descriptor: &RX_DESCRIPTORS[0],
            next_tx_descriptor: &TX_DESCRIPTORS[0],
        }
    }
}

impl<'a> Device<'a> for EthernetDevice {
    type RxToken = RxToken<'a>;
    type TxToken = TxToken<'a>;

    fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        if self.next_rx_descriptor.rdes0.read().own().bit_is_set() {
            return None;
        }
        if self.next_tx_descriptor.tdes0.read().own().bit_is_set() {
            return None;
        }

        Some((
            RxToken {
                emac0: &self.emac0,
                descriptor_pointer: &mut self.next_rx_descriptor,
            },
            TxToken {
                emac0: &self.emac0,
                descriptor_pointer: &mut self.next_tx_descriptor,
            },
        ))
    }

    fn transmit(&'a mut self) -> Option<Self::TxToken> {
        if self.next_tx_descriptor.tdes0.read().own().bit_is_set() {
            return None;
        }

        Some(TxToken {
            emac0: &self.emac0,
            descriptor_pointer: &mut self.next_tx_descriptor,
        })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut cap = DeviceCapabilities::default();

        cap.max_transmission_unit = ETHERNET_MTU;
        cap.max_burst_size = Some(NUM_TX_DESCRIPTORS);

        cap.checksum = ChecksumCapabilities::default();
        // cap.checksum.ipv4 = Checksum::None;
        // cap.checksum.ipv6 = Checksum::None;
        // cap.checksum.udp = Checksum::None;
        // cap.checksum.tcp = Checksum::None;
        // cap.checksum.icmpv4 = Checksum::None;
        // cap.checksum.icmpv6 = Checksum::None;

        cap
    }
}

pub struct RxToken<'a> {
    emac0: &'a EMAC0,
    descriptor_pointer: &'a mut &'static RDES,
}

impl<'a> phy::RxToken for RxToken<'a> {
    fn consume<R, F>(self, _timestamp: Instant, f: F) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        let descriptor = *self.descriptor_pointer;

        // We own the receive descriptor so check to see if it contains a valid frame.
        if descriptor.rdes0.read().bits() & DES0_RX_STAT_ERR == DES0_RX_STAT_ERR {
            descriptor.rdes0.write(|w| w.own().set_bit());
            return Err(smoltcp::Error::Checksum);
        }

        // We have a valid frame. First check that the "last descriptor" flag is set. We
        // sized the receive buffer such that it can always hold a valid frame so this
        // flag should never be clear at this point but...
        if descriptor.rdes0.read().bits() & DES0_RX_STAT_LAST_DESC != DES0_RX_STAT_LAST_DESC {
            descriptor.rdes0.write(|w| w.own().set_bit());
            return Err(smoltcp::Error::Truncated);
        }

        let len = ((descriptor.rdes0.read().bits() & DES0_RX_STAT_FRAME_LENGTH_M)
            >> DES0_RX_STAT_FRAME_LENGTH_S) as usize;
        assert!(len <= ETHERNET_MTU);
        let data =
            unsafe { core::slice::from_raw_parts_mut(descriptor.rdes2.read().bits() as *mut u8, len) };

        let result = f(data);

        descriptor.rdes0.write(|w| w.own().set_bit());
        self.emac0.rxpolld.write(|w| w);
        *self.descriptor_pointer = unsafe { &*(descriptor.rdes3.read().bits() as *const _) };
        result
    }
}

pub struct TxToken<'a> {
    emac0: &'a EMAC0,
    descriptor_pointer: &'a mut &'static TDES,
}

impl<'a> phy::TxToken for TxToken<'a> {
    fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        let descriptor = *self.descriptor_pointer;

        assert!(len <= ETHERNET_MTU);

        let data = unsafe {
            core::slice::from_raw_parts_mut(descriptor.tdes2.read().bits() as *mut u8, len)
        };
        let result = f(data);

        unsafe {
            descriptor
                .tdes1
                .write(|w| w.tbs1().bits(len.try_into().unwrap()));
        }

        descriptor.tdes0.modify(|_, w| w.own().set_bit());
        self.emac0.txpolld.write(|w| w);
        *self.descriptor_pointer = unsafe { &*(descriptor.tdes3.read().bits() as *const _) };
        result
    }
}
