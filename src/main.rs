#![no_main]
#![no_std]

// =====================================================================
// Use statements

// random number generator
// use nanorand::{pcg64::Pcg64, Rng, SeedableRng};

// cortex and microbit
use cortex_m_rt::entry;
use microbit::{
    board::{Board},
    display::blocking::Display,
    hal::{
        timer::Timer // timer
    }
};

// rtt_target
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// Game of Life engine - TBD
// mod life;
// use life::*;

// =====================================================================

// start of code
#[entry]
fn main() -> ! {
    // 
    rtt_init_print!();

    // define the board
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // RNG stuff
    // TBD

    // sample board - 
    let fb = [
        [1u8, 1u8, 1u8, 1u8, 1u8],
        [1u8, 0u8, 0u8, 0u8, 1u8],
        [1u8, 0u8, 0u8, 0u8, 1u8],
        [1u8, 0u8, 0u8, 0u8, 1u8],
        [1u8, 1u8, 1u8, 1u8, 1u8],
    ];

    // for accessing Button A
    // let left_button = board.buttons.button_a;

    // required loop
    // all behavior will take place inside the loop 
    loop {
        // light up the leds on the microbit
        display.show(&mut timer, fb, 1000);
        display.clear(); // clear the display
        timer.delay(2000); // delay
    }

}