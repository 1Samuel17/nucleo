#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod startup_nucleo;


// generic data to see in memory
static mut SCORES_GLOBAL: [u32; 5] = [1, 2, 3, 4, 5];
const _NUMBERS: [u32; 5] = [10, 20, 30, 40, 50];
static mut _BUFFER: [u8; 1024] = [0; 1024];

#[unsafe(no_mangle)]
fn main() {

    // generic code to see in memory
    let mut _total_score = 0;

    unsafe{
        for score in SCORES_GLOBAL {
        _total_score += score;
        }

    }

    loop {
        // Infinite loop to keep the program running
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
