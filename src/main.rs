#![no_main]
#![no_std]

// =====================================================================
// Use statements

// random number generator
use nanorand::{pcg64::Pcg64, Rng};

// cortex and microbit
use cortex_m_rt::entry;
use microbit::{
    board::{Board},
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer // timer
    }
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
    rprintln!("Starting Game of Life");

    // define the board
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // RNG stuff
    let mut hw_rng = HwRng::new(board.RNG);         // use the board's RNG to set up HW RNG
    let hal_seed = hw_rng.random_u64();             // generate a random u64
    let mut rng = Pcg64::new_seed(hal_seed.into()); // convert the random u64 into u128 and set up the new rng

    // sample board - it is currently a box
    let mut start_board = [
        [1u8, 1u8, 1u8, 1u8, 1u8],
        [1u8, 0u8, 0u8, 0u8, 1u8],
        [1u8, 0u8, 0u8, 0u8, 1u8],
        [1u8, 0u8, 0u8, 0u8, 1u8],
        [1u8, 1u8, 1u8, 1u8, 1u8],
    ];
    
    // Generate a random board
    for i in 0..5 {
        for j in 0..5 {
            let zero_or_one = rng.generate_range(0..=1); // choose either 0 or 1
            start_board[i][j] = zero_or_one; // if 1, light up the LED
        }

    }

    // for accessing Button A - TBD
    // let left_button = board.buttons.button_a;

    // required loop
    // all behavior will take place inside the loop 
    loop {
        // light up the leds on the microbit
        display.show(&mut timer, start_board, 1000);
        display.clear(); // clear the display

        // if all lights are off (zero), wait 5 frames for a button press and then generate a new random board
        if done(&start_board) {
            rprintln!("Board is all 0, resetting in 5 frames");
            timer.delay(2000); // delay

            // TBD - button press received

            // TBD - no button press received - generate a random board
            for i in 0..5 {
                for j in 0..5 {
                    let zero_or_one = rng.generate_range(0..=1);
                    start_board[i][j] = zero_or_one;
                }

            }
        }

        // run the game of life
        life(&mut start_board);
    }

}