#![no_main]
#![no_std]

// =====================================================================
// Use statements

// random number generator
use nanorand::{
    pcg64::Pcg64, 
    Rng
};

// cortex and microbit
use cortex_m_rt::entry;
use microbit::{
    board::{Board},             // for getting the board and its peripherals
    display::blocking::Display, // for lighting up the leds
    hal::{
        Rng as HwRng,           // for hardware rng 
        timer::Timer            // timer
    }
};

// for delaying in ms and checking button states
use embedded_hal::{
    delay::DelayNs, 
    digital::InputPin
};

// rtt_target
use panic_rtt_target as _;
use rtt_target::{
    rprintln, 
    rtt_init_print
};

// Game of Life engine
mod life;
use life::*;

// =====================================================================
// FUNCTIONS

/// Generate a random board. 
fn randomize_board(hw_rng: &mut HwRng, mut start_board: [[u8; 5]; 5]) -> [[u8; 5]; 5] {
    // choose a new random number and convert it to u128
    // every time this function is run, a new seed is generated
    let hal_seed = hw_rng.random_u64();             // generate a random u64
    let mut rng = Pcg64::new_seed(hal_seed.into()); // convert the random u64 into u128 and set up the new rng

    // randomly set parts of the board to 0 or 1
    // TODO - fix cargo clippy warnings
    for i in 0..5 {
        for j in 0..5 {
            start_board[i][j] = rng.generate_range(0..=1);
        }
    }

    // return the randomized board
    start_board
}


// =====================================================================

// start of code
#[entry]
fn main() -> ! {
    // initial rtt print
    rtt_init_print!();
    rprintln!("Starting Game of Life");

    // define the board
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // RNG stuff
    let mut hw_rng = HwRng::new(board.RNG);         // use the board's RNG to set up HW RNG

    // Generate a random board.
    let mut start_board: [[u8; 5]; 5] = [[0; 5]; 5];
    start_board = randomize_board(&mut hw_rng, start_board);

    // for accessing Button A.
    let mut left_button = board.buttons.button_a;
    // for accessing Button B. - TBD
    let mut right_button = board.buttons.button_b;

    // required loop
    // all behavior will take place inside the loop 
    loop {
        // light up the leds on the microbit every 100 ms
        display.show(&mut timer, start_board, 100);
        display.clear();                                // clear the display

        // if button a is pressed, randomize the board
        if left_button.is_low().unwrap() {
            rprintln!("Button A is pressed!");
            start_board = randomize_board(&mut hw_rng, start_board);
        } else { // Button B is pressed - turn 1s to 0s and 0s to 1s
            if right_button.is_low().unwrap() {
                rprintln!("Button B is pressed!");
                // make the complement of the board
                // TBD
            }
        }


        // if all lights are off (zero), wait 5 frames for a button press and then generate a new random board
        if done(&start_board) {
            rprintln!("Board is all 0, resetting in 5 frames");
            timer.delay_ms(500); // delay for 5 frames

            // TBD - button press received

            // no button press received - generate a random board
            start_board = randomize_board(&mut hw_rng, start_board);
        }

        // run the game of life
        rprintln!("Normal step");
        life(&mut start_board);
    }

}