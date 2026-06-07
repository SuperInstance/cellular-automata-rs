# cellular-automata-rs

Research-grade cellular automata library in pure Rust.

## Features

- **Elementary cellular automata**: All 256 Wolfram rules with configurable width and wrapping
- **Conway's Game of Life**: Arbitrary grid sizes with configurable neighbor rules
- **Langton's Ant**: Multiple ant support and color variants
- **Cyclic automata**: Configurable state count and neighborhood range

## Usage

```rust
use cellular_automata_rs::elementary::ElementaryCA;
use cellular_automata_rs::life::GameOfLife;
use cellular_automata_rs::langton::LangtonsAnt;
use cellular_automata_rs::cyclic::CyclicCA;

// Elementary CA - Rule 110
let mut ca = ElementaryCA::new(110, 79);
ca.set_single_center();
let state = ca.step();

// Game of Life
let mut gol = GameOfLife::new(20, 20);
gol.set(10, 10, true);
gol.step();

// Langton's Ant
let mut ant = LangtonsAnt::new(50, 50);
ant.step();

// Cyclic CA
let mut cyclic = CyclicCA::new(40, 40, 4);
cyclic.randomize();
cyclic.step();
```

## License

MIT OR Apache-2.0
