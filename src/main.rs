// Use statements
// Game of Life engine
mod life;
use life::*;

// 

// random number generator
use nanorand::{pcg64::Pcg64, Rng, SeedableRng};

// microbit
use microbit::{board::Board};

fn main() {
    println!("PLACEHOLDER");

    // Define an empty board.
    // All values are u8 as specified in life.rs.
    let mut fb = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // test that the board fb can be passed to the life function.
    life(&mut fb);

    // test that the board can be passed to the done function.
    let is_done = done(&fb);
    println!("{}",is_done);

    // test that a random number can be generated.
    let mut rng = nanorand::Pcg64::new_seed(1);
    let val: u8 = rng.generate();

    println!("Random num: {}", val);

}
