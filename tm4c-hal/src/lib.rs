#![no_std]

pub mod bb;
pub mod delay;
pub mod gpio;
pub mod serial;
pub mod sysctl;
pub mod time;

// I can't work out how to put this macro in a module.
#[macro_export]
macro_rules! gpio_macro {
    ($chip_crate:ident, $GPIOX:ident, $gpiox:ident, $iopd:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use super::*;
            use $chip_crate::$GPIOX;

            /// Provides mutual-exclusion for certain GPIO operations (such as
            /// selecting an alternate mode) that can't be done atomically.
            pub struct GpioControl {
                _0: (),
            }

            /// GPIO parts
            pub struct Parts {
                /// Pass an &mut reference to methods that require it.
                pub control: GpioControl,
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                /// Break this GPIO port into separate pins
                fn split(self, pc: &sysctl::PowerControl) -> Parts {
                    sysctl::control_power(
                        pc, sysctl::Domain::$iopd,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::Domain::$iopd);

                    Parts {
                        control: GpioControl { _0: () },
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Partially erased pin
            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> StatefulOutputPin for $PXx<Output<MODE>> where MODE: OutputMode {
                fn is_set_high(&self) -> bool {
                    let p = unsafe { &*$GPIOX::ptr() };
                    bb::read_bit(&p.data, self.i)
                }

                fn is_set_low(&self) -> bool {
                    !self.is_set_high()
                }
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> where MODE: OutputMode {
                fn set_high(&mut self) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.data, self.i, true); }
                }

                fn set_low(&mut self) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.data, self.i, false); }
                }
            }

            impl<MODE> InputPin for $PXx<Input<MODE>> where MODE: InputMode {
                fn is_high(&self) -> bool {
                    let p = unsafe { &*$GPIOX::ptr() };
                    bb::read_bit(&p.data, self.i)
                }

                fn is_low(&self) -> bool {
                    !self.is_high()
                }
            }

            impl<MODE> $PXx<Input<MODE>> where MODE: InputMode {
                /// Enables or disables interrupts on this GPIO pin.
                pub fn set_interrupt_mode(&mut self, mode: InterruptMode) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.im, self.i, false); }
                    match mode {
                        InterruptMode::LevelHigh => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS |= self.i;
                            unsafe { bb::change_bit(&p.is, self.i, true); }
                            // IBE &= ~self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, false); }
                            // IEV |= self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, true); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::LevelLow => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS |= self.i;
                            unsafe { bb::change_bit(&p.is, self.i, true); }
                            // IBE &= ~self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, false); }
                            // IEV &= ~self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, false); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::EdgeRising => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS &= ~self.i;
                            unsafe { bb::change_bit(&p.is, self.i, false); }
                            // IBE &= ~self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, false); }
                            // IEV |= self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, true); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::EdgeFalling => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS &= ~self.i;
                            unsafe { bb::change_bit(&p.is, self.i, false); }
                            // IBE &= ~self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, false); }
                            // IEV &= ~self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, false); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::EdgeBoth => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS &= ~self.i;
                            unsafe { bb::change_bit(&p.is, self.i, false); }
                            // IBE |= self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, true); }
                            // IEV |= self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, true); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::Disabled => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                        },
                    }
                }

                /// Marks the interrupt for this pin as handled. You should
                /// call this (or perform its functionality) from the ISR.
                pub fn clear_interrupt(&self) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.icr, self.i, true); }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> where MODE: IsUnlocked {
                    /// Configures the pin to serve as alternate function 1 through 15.
                    /// Disables open-drain to make the output a push-pull.
                    pub fn into_af_push_pull<AF>(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AlternateFunction<AF, PushPull>> where AF: AlternateFunctionChoice {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = AF::number() << ($i * 4);
                        unsafe {
                            p.pctl.modify(|r, w| w.bits((r.bits() & !mask) | bits));
                        }
                        unsafe { bb::change_bit(&p.afsel, $i, true); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to serve as alternate function 1 through 15.
                    /// Enables open-drain (useful for I2C SDA, for example).
                    pub fn into_af_open_drain<AF, ODM>(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AlternateFunction<AF, OpenDrain<ODM>>> where AF: AlternateFunctionChoice, ODM: OpenDrainMode {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = AF::number() << ($i * 4);
                        unsafe {
                            p.pctl.modify(|r, w| w.bits((r.bits() & !mask) | bits));
                        }
                        unsafe { bb::change_bit(&p.afsel, $i, true); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, true); }
                        unsafe { bb::change_bit(&p.pur, $i, ODM::pup()); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self
                    ) -> $PXi<Input<Floating>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self
                    ) -> $PXi<Input<PullDown>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, true); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self
                    ) -> $PXi<Input<PullUp>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, true); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output<ODM>(
                        self
                    ) -> $PXi<Output<OpenDrain<ODM>>> where ODM: OpenDrainMode {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, true); }
                        unsafe { bb::change_bit(&p.odr, $i, true); }
                        unsafe { bb::change_bit(&p.pur, $i, ODM::pup()); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self
                    ) -> $PXi<Output<PushPull>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, true); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin as tri-state
                    pub fn into_tri_state(
                        self
                    ) -> $PXi<Tristate> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.den, $i, false); }
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        $PXi { _mode: PhantomData }
                    }

                }

                impl<MODE> $PXi<MODE> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<MODE> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> where MODE: OutputMode {
                    fn is_set_high(&self) -> bool {
                        let p = unsafe { &*$GPIOX::ptr() };
                        bb::read_bit(&p.data, $i)
                    }

                    fn is_set_low(&self) -> bool {
                        !self.is_set_high()
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> where MODE: OutputMode {
                    fn set_high(&mut self) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.data, $i, true); }
                    }

                    fn set_low(&mut self) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.data, $i, false); }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> where MODE: InputMode {
                    fn is_high(&self) -> bool {
                        let p = unsafe { &*$GPIOX::ptr() };
                        bb::read_bit(&p.data, $i)
                    }

                    fn is_low(&self) -> bool {
                        !self.is_high()
                    }
                }

                impl<MODE> $PXi<Input<MODE>> where MODE: InputMode {
                    /// Enables or disables interrupts on this GPIO pin.
                    pub fn set_interrupt_mode(&mut self, mode: InterruptMode) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.im, $i, false); }
                        match mode {
                            InterruptMode::LevelHigh => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS |= $i;
                                unsafe { bb::change_bit(&p.is, $i, true); }
                                // IBE &= ~$i;
                                unsafe { bb::change_bit(&p.ibe, $i, false); }
                                // IEV |= $i;
                                unsafe { bb::change_bit(&p.iev, $i, true); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::LevelLow => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS |= $i;
                                unsafe { bb::change_bit(&p.is, $i, true); }
                                // IBE &= ~$i;
                                unsafe { bb::change_bit(&p.ibe, $i, false); }
                                // IEV &= ~$i;
                                unsafe { bb::change_bit(&p.iev, $i, false); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::EdgeRising => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS &= ~$i;
                                unsafe { bb::change_bit(&p.is, $i, false); }
                                // IBE &= ~$i;
                                unsafe { bb::change_bit(&p.ibe, $i, false); }
                                // IEV |= $i;
                                unsafe { bb::change_bit(&p.iev, $i, true); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::EdgeFalling => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS &= ~$i;
                                unsafe { bb::change_bit(&p.is, $i, false); }
                                // IBE &= ~$i;
                                unsafe { bb::change_bit(&p.ibe, $i, false); }
                                // IEV &= ~$i;
                                unsafe { bb::change_bit(&p.iev, $i, false); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::EdgeBoth => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS &= ~$i;
                                unsafe { bb::change_bit(&p.is, $i, false); }
                                // IBE |= $i;
                                unsafe { bb::change_bit(&p.ibe, $i, true); }
                                // IEV |= $i;
                                unsafe { bb::change_bit(&p.iev, $i, true); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::Disabled => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                            },
                        }
                    }

                    /// Marks the interrupt for this pin as handled. You should
                    /// call this (or perform its functionality) from the ISR.
                    pub fn clear_interrupt(&self) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.icr, $i, true); }
                    }
                }

                impl $PXi<Locked> {
                    /// Unlock a GPIO so that it can be used. This is required
                    /// on 'special' GPIOs that the manufacturer doesn't want
                    /// you to change by accident - like NMI and JTAG pins.
                    pub fn unlock(self, _gpio_control: &mut GpioControl) -> $PXi<Tristate> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        p.lock.write(|w| w.lock().key());
                        p.cr.modify(|_, w| unsafe { w.bits(1 << $i) });
                        p.lock.write(|w| w.lock().unlocked());
                        unsafe { bb::change_bit(&p.den, $i, false); }
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        $PXi { _mode: PhantomData }
                    }
                }
            )+
        }
    }
}

#[macro_export]
macro_rules! uart_hal_macro {
    ($(
        $UARTX:ident: ($powerDomain:ident, $uartX:ident),
    )+) => {
        $(
            impl<TX, RX, RTS, CTS> Serial<$UARTX, TX, RX, RTS, CTS> {
                /// Configures a UART peripheral to provide serial communication
                pub fn $uartX(
                    mut uart: $UARTX,
                    tx_pin: TX,
                    rx_pin: RX,
                    mut rts_pin: RTS,
                    mut cts_pin: CTS,
                    baud_rate: Bps,
                    nl_mode: NewlineMode,
                    clocks: &Clocks,
                    pc: &sysctl::PowerControl
                ) -> Self
                where
                    TX: TxPin<$UARTX>,
                    RX: RxPin<$UARTX>,
                    CTS: CtsPin<$UARTX>,
                    RTS: RtsPin<$UARTX>,
                {
                    // Enable UART peripheral clocks
                    sysctl::control_power(
                        pc, sysctl::Domain::$powerDomain,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::Domain::$powerDomain);

                    // Reset UART
                    uart.ctl.reset();

                    // Calculate baud rate dividers
                    let baud_int: u32 = (((clocks.sysclk.0 * 8) / baud_rate.0) + 1) / 2;

                    // Set baud rate
                    uart.ibrd.write(|w|
                        unsafe { w.divint().bits((baud_int / 64) as u16) });
                    uart.fbrd.write(|w|
                        unsafe { w.divfrac().bits((baud_int % 64) as u8) });

                    // Set data bits / parity / stop bits / enable fifo
                    uart.lcrh.write(|w| w.wlen()._8().fen().bit(true));

                    // Activate flow control (if desired)
                    rts_pin.enable(&mut uart);
                    cts_pin.enable(&mut uart);

                    // Enable uart
                    uart.ctl.modify(|_, w| w.rxe().bit(true).txe().bit(true).uarten().bit(true));

                    Serial { uart, tx_pin, rx_pin, rts_pin, cts_pin, nl_mode }
                }

                /// Change the current baud rate for the UART. We need the
                /// `clocks` object in order to calculate the magic baud rate
                /// register values.
                pub fn change_baud_rate(&mut self, baud_rate: Bps, clocks: &Clocks) {
                    // Calculate baud rate dividers
                    let baud_int: u32 = (((clocks.sysclk.0 * 8) / baud_rate.0) + 1) / 2;
                    // Set baud rate
                    self.uart.ibrd.write(|w|
                        unsafe { w.divint().bits((baud_int / 64) as u16) });
                    self.uart.fbrd.write(|w|
                        unsafe { w.divfrac().bits((baud_int % 64) as u8) });
                }

                /// Splits the `Serial` abstraction into a transmitter and a
                /// receiver half. If you do this you can transmit and receive
                /// in different threads.
                pub fn split(self) -> (Tx<$UARTX, TX, RTS>, Rx<$UARTX, RX, CTS>) {
                    (
                        Tx {
                            uart: self.uart,
                            pin: self.tx_pin,
                            nl_mode: self.nl_mode,
                            flow_pin: self.rts_pin,
                        },
                        Rx {
                            _uart: PhantomData,
                            pin: self.rx_pin,
                            flow_pin: self.cts_pin,
                        },
                    )
                }

                /// Write a complete string to the UART.
                /// If this returns `Ok(())`, all the data was sent.
                /// Otherwise you get number of octets sent and the error.
                pub fn write_all<I: ?Sized>(&mut self, data: &I)
                where
                    I: AsRef<[u8]>,
                {
                    for octet in data.as_ref().iter() {
                        block!(self.write(*octet)).unwrap(); // E = Void
                    }
                }

                /// Re-combine a split UART
                pub fn combine(tx: Tx<$UARTX, TX, RTS>, rx: Rx<$UARTX, RX, CTS>) -> Serial<$UARTX, TX, RX, RTS, CTS> {
                    Serial {
                        uart: tx.uart,
                        nl_mode: tx.nl_mode,
                        rx_pin: rx.pin,
                        tx_pin: tx.pin,
                        rts_pin: tx.flow_pin,
                        cts_pin: rx.flow_pin,
                    }
                }

                /// Releases the UART peripheral and associated pins
                pub fn free(self) -> ($UARTX, TX, RX, RTS, CTS) {
                    (self.uart, self.tx_pin, self.rx_pin, self.rts_pin, self.cts_pin)
                }
            }

            impl<TX, RTS> Tx<$UARTX, TX, RTS> {
                /// Write a complete string to the UART.
                /// If this returns `Ok(())`, all the data was sent.
                /// Otherwise you get number of octets sent and the error.
                pub fn write_all<I: ?Sized>(&mut self, data: &I)
                where
                    I: AsRef<[u8]>,
                {
                    for octet in data.as_ref().iter() {
                        block!(self.write(*octet)).unwrap(); // E = Void
                    }
                }
            }

            impl<TX, RX, RTS, CTS> serial::Read<u8> for Serial<$UARTX, TX, RX, RTS, CTS> {
                type Error = Void;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    if self.uart.fr.read().rxfe().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(self.uart.dr.read().data().bits())
                }
            }

            impl<RX, CTS> serial::Read<u8> for Rx<$UARTX, RX, CTS> {
                type Error = Void;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    // We're only doing RX operations here so this is safe.
                    let p = unsafe { &*$UARTX::ptr() };
                    if p.fr.read().rxfe().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(p.dr.read().data().bits())
                }
            }

            impl<TX, RX, RTS, CTS> serial::Write<u8> for Serial<$UARTX, TX, RX, RTS, CTS> {
                type Error = Void;

                fn flush(&mut self) -> nb::Result<(), Void> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(())
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    self.uart.dr.write(|w| unsafe { w.data().bits(byte) });
                    Ok(())
                }
            }

            impl<TX, RTS> serial::Write<u8> for Tx<$UARTX, TX, RTS> {
                type Error = Void;

                fn flush(&mut self) -> nb::Result<(), Void> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(())
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    self.uart.dr.write(|w| unsafe { w.data().bits(byte) });
                    Ok(())
                }
            }

            /// Allows the Uart to be passed to 'write!()' and friends.
            impl<TX, RX, RTS, CTS> fmt::Write for Serial<$UARTX, TX, RX, RTS, CTS> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    match self.nl_mode {
                        NewlineMode::Binary => self.write_all(s),
                        NewlineMode::SwapLFtoCRLF => {
                            for byte in s.bytes() {
                                if byte == 0x0A {
                                    // Prefix every \n with a \r
                                    block!(self.write(0x0D)).unwrap(); // E = Void
                                }
                                block!(self.write(byte)).unwrap(); // E = Void
                            }
                        }
                    }
                    Ok(())
                }
            }

            /// Allows the Tx to be passed to 'write!()' and friends.
            impl<TX, RTS> fmt::Write for Tx<$UARTX, TX, RTS> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    match self.nl_mode {
                        NewlineMode::Binary => self.write_all(s),
                        NewlineMode::SwapLFtoCRLF => {
                            for byte in s.bytes() {
                                if byte == 0x0A {
                                    // Prefix every \n with a \r
                                    block!(self.write(0x0D)).unwrap(); // E = Void
                                }
                                block!(self.write(byte)).unwrap(); // E = Void
                            }
                        }
                    }
                    Ok(())
                }
            }

        )+
    }
}

#[macro_export]
macro_rules! uart_pin_macro {
    ($UARTn:ident,
        cts: [$(($($ctsgpio: ident)::*, $ctsaf: ident)),*],
        // dcd: [() $(, ($($dcdgpio: ident)::*, $dcdaf: ident))*],
        // dsr: [() $(, ($($dsrgpio: ident)::*, $dsraf: ident))*],
        // dtr: [() $(, ($($dtrgpio: ident)::*, $dtraf: ident))*],
        // ri: [() $(, ($($rigpio: ident)::*, $riaf: ident))*],
        rts: [$(($($rtsgpio: ident)::*, $rtsaf: ident)),*],
        rx: [$(($($rxgpio: ident)::*, $rxaf: ident)),*],
        tx: [$(($($txgpio: ident)::*, $txaf: ident)),*],
    ) => {
        $(
            unsafe impl<T> CtsPin<$UARTn> for $($ctsgpio)::*<AlternateFunction<$ctsaf, T>>
            where
                T: OutputMode,
            {
                fn enable(&mut self, uart: &mut $UARTn) {
                    uart.ctl.modify(|_, w| w.ctsen().set_bit());
                }
            }
        )*

        $(
            unsafe impl<T> RtsPin<$UARTn> for $($rtsgpio)::*<AlternateFunction<$rtsaf, T>>
            where
                T: OutputMode,
            {
                fn enable(&mut self, uart: &mut $UARTn) {
                    uart.ctl.modify(|_, w| w.rtsen().set_bit());
                }
            }
        )*

        $(
            unsafe impl <T> RxPin<$UARTn> for $($rxgpio)::*<AlternateFunction<$rxaf, T>>
            where
                T: OutputMode,
            {}
        )*

        $(
            unsafe impl <T> TxPin<$UARTn> for $($txgpio)::*<AlternateFunction<$txaf, T>>
            where
                T: OutputMode,
            {}
        )*
    }
}
