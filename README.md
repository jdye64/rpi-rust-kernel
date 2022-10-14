# BCM2837 Rust Kernel

This project is a simple and non practical example for my personal learning of how to write a baremetal Rust kernel for the BCM2837 chip (AKA Raspberr PI).

This example kernel simply demonstrates a "blinking LED". To accomplish this of course a LED must be attached to GPIO pin 21 on your Raspberry PI device through a breadboard and a 470 ohm resistor back to GROUND on the Raspberry PI device.

## Assumptions
- Rust is installed on your development environment
- If on Ubuntu you have `binutils-arm-non-eabi` installed or the equivalent for your development machine operating system. You can installed on Ubuntu by running `sudo apt install binutils-arm-none-eabi`

## Building Kernel
- Add the rust target for armv7a - `rustup target add armv7a-none-eabi`
- Compile the arm target - `cargo rustc -- -C link-arg=--script=./linker.ld`, this custom linker script is required because the bootcode firmware expects the first instruction to being at 0x8000. This is out of our control so we must ensure we abide by those constraints. Likewise further details about pins and settings can be found at https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf
At this point you are left with a ELF file named `my-rusty-pi`, this file is understood by Linux. However, for embedded development we need to export the executable contents of the ELF to a purely binary file. We do this by running `arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/my-rusty-pi ./kernel7.img`

## Deploying to RPI
The `kernel7.img` file along with `fixup.dat`, `start.elf`, and `bootcode.bin` (which were pulled from https://github.com/raspberrypi/firmware/tree/master/boot) can be written to a SD card and inserted into the Raspberry PI. If you have completely your wiring correctly when you boot the RPI device the LED light should blink as expected.