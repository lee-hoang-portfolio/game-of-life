# Life
# Name: Lee Hoang

# What I did

I built a Rust program that plays [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) on the Microbit v2.

## Features

- Follows Game of Life rules
- Press and hold the A button to start from a new random pattern.
- Press the B button to invert the pattern. Lights that are on will turn off and lights that are off will turn on.

## How to run

To build and run this code, run:

```
cargo build
cargo embed
```

# How it went

TBD

# Observations

While I was working on this program, I observed the following behaviors:

## Stuck on one pattern or alternating between two patterns

Sometimes, the game of life will get stuck in a state where it only shows one pattern or goes back and forth between two patterns. 

The only way to get out of this state is to press either the A or B button.

## Cells disappear immediately after pressing the B Button

When the B button is pressed, the cells invert before they all disappear.