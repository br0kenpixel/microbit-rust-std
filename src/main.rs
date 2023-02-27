#![no_main]
#![feature(restricted_std)]

use std::thread::sleep;
use std::time::Duration;
use cortex_m_rt::entry;
use microbit::board::Board;
use cortex_m_semihosting::hprintln;

use alloc_cortex_m::CortexMHeap;
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
    let _nums = vec![1, 2, 3];
    let _text = String::from("Hello World!");
    let _board = Board::take().unwrap();

    loop {
        hprintln!("Hello!");
        hprintln!("Vector: {_nums}");
        hprintln!("String: {_text}");
        sleep(Duration::from_secs(3));
    }
}