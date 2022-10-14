#![no_std]
#![no_main]

//3F20_0008 fsel2 1<<3 turn pin21 into an output
//3F20_001C gpio1_set 1<<21 turns pin 21 on
//3F20_0028 gpio1_clear 1<<21 turns pin 21 off

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // turn PIN21 into an output
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1<<3);

        loop {
            // Turn pin on
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1<<21);

            for _ in 1..50000 {
                asm!("nop");
            }

            // Turn pin off
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1<<21);

            for _ in 1..50000 {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop {}
}