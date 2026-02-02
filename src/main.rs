#![no_main]
#![no_std]

// =====================================================================
// Use statements

// random number generator
use nanorand::{
    Rng,
    // Docs: https://docs.rs/nanorand/latest/nanorand/
    pcg64::Pcg64,
};

// cortex and microbit
use cortex_m_rt::entry; // Docs: https://docs.rs/cortex-m-rt/0.7.5/cortex_m_rt/ and https://docs.rs/cortex-m-rt/0.7.0/cortex_m_rt/
use microbit::{
    // Docs: https://docs.rs/microbit-v2/0.16.0/microbit/index.html
    board::Board,               // for getting the board and its peripherals
    display::blocking::Display, // for lighting up the leds
    hal::{
        Rng as HwRng, // for hardware rng
        timer::Timer, // timer
    },
};

// for delaying in ms and checking button states
use embedded_hal::{
    // Docs: https://docs.rs/embedded-hal/1.0.0/embedded_hal/index.html
    delay::DelayNs,
    digital::InputPin,
};

// rtt_target
// Docs: https://docs.rs/crate/rtt-target/0.6.0
// https://docs.rs/panic-rtt-target/latest/panic_rtt_target/
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// Game of Life engine
mod life;
use life::*;

// =====================================================================
// FUNCTIONS

/// Generate a random board.
fn randomize_board(hw_rng: &mut HwRng, mut board: [[u8; 5]; 5]) -> [[u8; 5]; 5] {
    // choose a new random number and convert it to u128
    // every time this function is run, a new seed is generated
    let hal_seed = hw_rng.random_u64(); // generate a random u64
    let mut rng = Pcg64::new_seed(hal_seed.into()); // convert the random u64 into u128 and set up the new rng

    // randomly set parts of the board to 0 or 1
    for row in &mut board {
        for led in row {
            *led = rng.generate_range(0..=1);
        }
    }

    // return the randomized board
    board
}

/// invert the board - this means turn off leds that are lit and turn on leds that are off
fn invert_board(mut board: [[u8; 5]; 5]) -> [[u8; 5]; 5] {
    // turn leds off or on depending on their initial value
    for row in &mut board {
        for led in row {
            if *led == 0u8 {
                // dereference the led variable to access the u8 value
                *led = 1u8;
            } else if *led == 1u8 {
                *led = 0u8;
            }
        }
    }

    // return the board with inverted lights
    board
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
    let mut hw_rng = HwRng::new(board.RNG); // use the board's RNG to set up HW RNG

    // Generate a random board.
    let mut current_board: [[u8; 5]; 5] = [[0; 5]; 5];
    current_board = randomize_board(&mut hw_rng, current_board);

    // for accessing Button A.
    let mut left_button = board.buttons.button_a;
    // for accessing Button B.
    let mut right_button = board.buttons.button_b;

    // required loop
    // all behavior will take place inside the loop
    loop {
        // light up the leds on the microbit every 100 ms
        display.show(&mut timer, current_board, 100);
        display.clear();

        // if button a is pressed, randomize the board
        if left_button.is_low().unwrap() {
            rprintln!("Button A is pressed!");
            current_board = randomize_board(&mut hw_rng, current_board);
        } else if right_button.is_low().unwrap() && left_button.is_high().unwrap() {
            // Button B is pressed - turn 1s to 0s and 0s to 1s
            rprintln!("Button B is pressed!");
            // invert the lights on the board
            current_board = invert_board(current_board);
            display.show(&mut timer, current_board, 100); // show the inverted board
            timer.delay_ms(500); // ignore the b button for 5 seconds
        }

        // if all lights are off (zero), wait 5 frames for a button press and then generate a new random board
        if done(&current_board) {
            rprintln!("Board is all 0, resetting in 5 frames");
            timer.delay_ms(500); // delay for 5 frames

            // no button press received - generate a random board
            if left_button.is_high().unwrap() && right_button.is_high().unwrap() {
                current_board = randomize_board(&mut hw_rng, current_board);
            }
        }

        // run the game of life
        life(&mut current_board);
    }
}
