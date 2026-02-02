# Life
# Name: Lee Hoang

# What I did

I built a Rust program that plays [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) on the Microbit v2.

## Features

- Follows Game of Life rules
- Press and hold the A button to start from a new random pattern.
- Press the B button to invert the pattern. Lights that are on will turn off and lights that are off will turn on.

## How to use

### Cloning the project

Open a terminal on your computer (in either WSL2 or your main OS) and clone the project using the following command: 

```
git clone https://github.com/lee-hoang-portfolio/game-of-life.git
```

### Setting up a Microbit v2 connection

First, the Microbit v2 needs to be connected via USB to a computer. If you are using WSL2, you will need to follow the steps in this [Microsoft article](https://learn.microsoft.com/en-us/windows/wsl/connect-usb) to set up `usbipd` and in the [MB2 Discovery textbook](https://docs.rust-embedded.org/discovery-mb2/03-setup/index.html) to set up `probe-rs` rules.  

### Building and running the program

Once the setup is complete, you can build and run this code by typing the following commands in the project root directory:

```
cargo build
cargo embed
```

`cargo build` compiles the program while `cargo embed` flashes the program onto the Microbit.

# How it went

Working on this project was not too difficult. Once I got past the initial setup issues, most of my difficulty was in figuring out the Microbit v2 code to make the board play the Game of Life.

## Setup issues

My initial biggest issue was getting WSL2 to work with my Microbit v2. To start, I followed this [Microsoft article](https://learn.microsoft.com/en-us/windows/wsl/connect-usb) to set up connectivity between WSL2 and my Microbit v2. 

## cargo clippy warnings

When I encountered `cargo clippy` warnings, I followed cargo's suggestions to use an iterator and iterate over the LED array directly. 

## Highlights

It was exciting to see the board light up and display the patterns. Testing the buttons and seeing them work was also a highlight.  

# Observations

While I was working on this program, I observed the following behaviors:

## Walls, floors, and ceilings

Columns of LEDs would sometimes light up such that they formed vertical walls. The same can be said for rows of LEDs on the top and bottom of the board. 

## Stuck on one pattern or alternating between two patterns

Sometimes, the program will get stuck in a state where it only shows one pattern or goes back and forth between two patterns. 

The only way to get out of this state is to press either the A or B button. In this state, pressing the B button lets you see an inverted version of the still pattern or alternating pattern.

## Cells disappear immediately after pressing the B Button

When the B button is pressed, the cells invert before they all disappear. One of the rules mentioned in the Wikipedia article states that cells may disappear due to overpopulation. 

# Acknowledgements

- [MB2 Discovery Book](https://docs.rust-embedded.org/discovery-mb2/index.html) - provided starting points for Microbit v2 code.
- [pdx-cs-rust-embedded](https://github.com/pdx-cs-rust-embedded) - provided starting points for setting up the project.