#![no_main]
#![no_std]

// =====================================================================
// Use statements

// random number generator
use nanorand::{pcg64::Pcg64, Rng, SeedableRng};

// cortex and microbit
use cortex_m_rt::entry;
use microbit::{
    board::{Board, Buttons}
};

// rtt_target
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// Game of Life engine
mod life;
use life::*;

// =====================================================================

// start of code
#[entry]
fn main() -> ! {
    // 
    rtt_init_print!();

    // define the board
    let board = Board::take().unwrap();

    // required loop
    // all behavior will take place inside the loop 
    loop {
        rprintln!("This is a placeholder");
    }

}