#![no_main]
#![feature(restricted_std)]

use cortex_m_rt::entry;
use microbit::{board::Board, hal::prelude::*, hal::Timer};
use rtt_target::{rprintln, rtt_init_print};

use alloc_cortex_m::CortexMHeap;
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    rtt_init_print!();
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let _nums = vec![1, 2, 3];
    let _text = String::from("Hello World!");

    loop {
        rprintln!("Hello!");
        rprintln!("Vector: {:?}", _nums);
        rprintln!("String: {}", _text);
        timer.delay_ms(1000u16);
    }
}
