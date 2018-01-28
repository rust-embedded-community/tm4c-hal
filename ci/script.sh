# This script takes care of testing your crate

set -ex

main() {
    case $TARGET in
        x86_64-unknown-linux-gnu)
            cargo check --target $TARGET
            ;;
        *)
            xargo check --target $TARGET
            xargo check --target $TARGET --features rt
            ;;
    esac

}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
