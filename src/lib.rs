//! # cellular-automata-rs
//!
//! A research-grade cellular automata library implementing elementary CA (Wolfram rules 0-255),
//! Conway's Game of Life, Langton's Ant, and cyclic automata.
//!
//! # Modules
//!
//! - [`elementary`] — Elementary (1D) cellular automata with Wolfram rule numbering
//! - [`life`] — Conway's Game of Life and 2D cellular automata
//! - [`langton`] — Langton's Ant and multi-ant variants
//! - [`cyclic`] — Cyclic cellular automata
//! - [`rule`] — Rule parsing and bit manipulation utilities

pub mod cyclic;
pub mod elementary;
pub mod langton;
pub mod life;
pub mod rule;

pub use cyclic::CyclicCA;
pub use elementary::ElementaryCA;
pub use langton::LangtonsAnt;
pub use life::GameOfLife;
