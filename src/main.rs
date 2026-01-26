mod life;
use life::*;

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

}
