//! Elementary (1D) cellular automata with Wolfram rule numbering (0-255).
//!
//! This module provides a configurable 1D cellular automaton with periodic boundary
//! conditions (toroidal wrapping). The state is a vector of booleans representing
//! alive/dead cells.

use crate::rule::Rule;

/// An elementary (1D) cellular automaton governed by a Wolfram rule.
///
/// The automaton operates on a 1D grid of cells with periodic boundary conditions.
/// Each cell is either alive (`true`) or dead (`false`).
///
/// # Examples
///
/// ```
/// use cellular_automata_rs::elementary::ElementaryCA;
///
/// let mut ca = ElementaryCA::new(110, 21);
/// ca.set_single_center();
/// let state = ca.step();
/// assert_eq!(state.len(), 21);
/// ```
#[derive(Debug, Clone)]
pub struct ElementaryCA {
    /// The Wolfram rule governing this automaton.
    pub rule: Rule,
    /// Current state of all cells.
    pub state: Vec<bool>,
}

impl ElementaryCA {
    /// Create a new elementary CA with the given rule number and width.
    ///
    /// All cells start dead (`false`). Use [`set_single_center`] or
    /// [`set_state`] to initialize.
    ///
    /// # Panics
    ///
    /// Panics if `width` is 0.
    pub fn new(rule_number: u8, width: usize) -> Self {
        assert!(width > 0, "Width must be positive");
        Self {
            rule: Rule::new(rule_number),
            state: vec![false; width],
        }
    }

    /// Set a single alive cell in the center, all others dead.
    ///
    /// For even widths, the center-left cell is used.
    pub fn set_single_center(&mut self) {
        let center = self.state.len() / 2;
        self.state.iter_mut().for_each(|c| *c = false);
        self.state[center] = true;
    }

    /// Set the full state from a slice of booleans.
    ///
    /// # Panics
    ///
    /// Panics if the slice length differs from the automaton width.
    pub fn set_state(&mut self, state: &[bool]) {
        assert_eq!(state.len(), self.state.len(), "State length must match width");
        self.state.copy_from_slice(state);
    }

    /// Advance the automaton by one generation using periodic boundary conditions.
    ///
    /// Returns the new state.
    pub fn step(&mut self) -> &[bool] {
        let n = self.state.len();
        let mut next = vec![false; n];
        for (i, slot) in next.iter_mut().enumerate() {
            let left = self.state[(i + n - 1) % n];
            let center = self.state[i];
            let right = self.state[(i + 1) % n];
            *slot = self.rule.apply(left, center, right);
        }
        self.state = next;
        &self.state
    }

    /// Advance the automaton by `n` generations.
    ///
    /// Returns the final state.
    pub fn step_n(&mut self, n: usize) -> &[bool] {
        for _ in 0..n {
            self.step();
        }
        &self.state
    }

    /// Get the current state as a string with `█` for alive and `·` for dead.
    pub fn to_string_visual(&self) -> String {
        self.state
            .iter()
            .map(|&alive| if alive { '█' } else { '·' })
            .collect()
    }

    /// Get the current state as a string with `1` for alive and `0` for dead.
    pub fn to_string_binary(&self) -> String {
        self.state
            .iter()
            .map(|&alive| if alive { '1' } else { '0' })
            .collect()
    }

    /// Count the number of alive cells.
    pub fn alive_count(&self) -> usize {
        self.state.iter().filter(|&&b| b).count()
    }

    /// Get the width of the automaton.
    pub fn width(&self) -> usize {
        self.state.len()
    }

    /// Check if the current state is all dead.
    pub fn is_all_dead(&self) -> bool {
        !self.state.iter().any(|&b| b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ca_initial_state() {
        let ca = ElementaryCA::new(30, 10);
        assert_eq!(ca.state.len(), 10);
        assert!(ca.state.iter().all(|&b| !b));
        assert!(ca.is_all_dead());
    }

    #[test]
    fn test_set_single_center_odd() {
        let mut ca = ElementaryCA::new(30, 5);
        ca.set_single_center();
        assert_eq!(ca.state, vec![false, false, true, false, false]);
    }

    #[test]
    fn test_set_single_center_even() {
        let mut ca = ElementaryCA::new(30, 4);
        ca.set_single_center();
        // center = 4/2 = 2
        assert_eq!(ca.state, vec![false, false, true, false]);
    }

    #[test]
    #[should_panic(expected = "Width must be positive")]
    fn test_new_ca_zero_width() {
        ElementaryCA::new(30, 0);
    }

    #[test]
    fn test_rule_0_kills_everything() {
        let mut ca = ElementaryCA::new(0, 5);
        ca.set_single_center();
        ca.step();
        assert!(ca.is_all_dead());
    }

    #[test]
    fn test_rule_255_keeps_everything() {
        let mut ca = ElementaryCA::new(255, 3);
        ca.set_single_center();
        ca.step();
        // With wrapping and rule 255, everything becomes alive
        assert_eq!(ca.alive_count(), 3);
    }

    #[test]
    fn test_rule_30_single_center_evolution() {
        let mut ca = ElementaryCA::new(30, 7);
        ca.set_single_center();
        // Initial: [0,0,0,1,0,0,0]
        // Rule 30: 00011110
        // pattern 001 (idx=2,3): 1, pattern 010 (idx=1,3): 1, pattern 100 (idx=3): 1, pattern 000: 0
        ca.step();
        // From single center: [1,1,1,0,1,1,0] - no wrapping needed for width 7
        // Actually: idx 2 sees [0,0,1]->1, idx 3 sees [0,1,0]->1, idx 4 sees [1,0,0]->1
        // idx 1 sees [0,0,0]->0, but with wrapping idx 0 sees [0,0,0]->0, idx 6 sees [0,0,0]->0
        // Let me just count: cells 2,3,4 should be alive
        assert_eq!(ca.alive_count(), 3);
    }

    #[test]
    fn test_rule_110_not_trivially_dead() {
        let mut ca = ElementaryCA::new(110, 7);
        ca.set_single_center();
        for _ in 0..5 {
            ca.step();
        }
        // Rule 110 is known to produce complex patterns
        assert!(!ca.is_all_dead());
    }

    #[test]
    fn test_rule_184_traffic_model() {
        // Rule 184 models traffic flow: 10111000
        let mut ca = ElementaryCA::new(184, 6);
        ca.set_state(&[true, false, true, false, true, false]);
        ca.step();
        // With periodic boundary: cars move right if space ahead is empty
        // [1,0,1,0,1,0] -> cars at 0,2,4 check right (1,3,5), all empty -> move
        // Result: [0,1,0,1,0,1]
        assert_eq!(ca.state, vec![false, true, false, true, false, true]);
    }

    #[test]
    fn test_step_n() {
        let mut ca1 = ElementaryCA::new(30, 11);
        ca1.set_single_center();
        ca1.step_n(3);

        let mut ca2 = ElementaryCA::new(30, 11);
        ca2.set_single_center();
        ca2.step();
        ca2.step();
        ca2.step();

        assert_eq!(ca1.state, ca2.state);
    }

    #[test]
    fn test_periodic_boundary() {
        let mut ca = ElementaryCA::new(170, 5);
        // Rule 170 is left shift: output = right neighbor
        ca.set_state(&[true, false, false, false, false]);
        ca.step();
        // With periodic wrapping: each cell gets its right neighbor
        // [0,0,0,0,1]
        assert_eq!(ca.state, vec![false, false, false, false, true]);
    }

    #[test]
    fn test_visual_string() {
        let mut ca = ElementaryCA::new(30, 3);
        ca.set_state(&[true, false, true]);
        assert_eq!(ca.to_string_visual(), "█·█");
    }

    #[test]
    fn test_binary_string() {
        let mut ca = ElementaryCA::new(30, 3);
        ca.set_state(&[true, false, true]);
        assert_eq!(ca.to_string_binary(), "101");
    }

    #[test]
    fn test_alive_count() {
        let mut ca = ElementaryCA::new(30, 5);
        ca.set_state(&[true, false, true, false, true]);
        assert_eq!(ca.alive_count(), 3);
    }

    #[test]
    fn test_width() {
        let ca = ElementaryCA::new(30, 42);
        assert_eq!(ca.width(), 42);
    }

    #[test]
    fn test_set_state_length_mismatch() {
        let mut ca = ElementaryCA::new(30, 5);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            ca.set_state(&[true; 3]);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_rule_90_sierpinski() {
        // Rule 90 produces a Sierpinski triangle from a single cell
        let mut ca = ElementaryCA::new(90, 15);
        ca.set_single_center();
        // After a few steps, we should still have alive cells
        for _ in 0..4 {
            ca.step();
        }
        assert!(!ca.is_all_dead());
        // Rule 90 is XOR of neighbors: symmetric from single seed
    }

    #[test]
    fn test_clone_independence() {
        let mut ca = ElementaryCA::new(30, 5);
        ca.set_single_center();
        let clone = ca.clone();
        ca.step();
        // Clone should not have changed
        assert_eq!(clone.state, vec![false, false, true, false, false]);
    }

    #[test]
    fn test_rule_254_grows() {
        // Rule 254: 11111110 -> everything becomes 1 except 000
        let mut ca = ElementaryCA::new(254, 5);
        ca.set_single_center();
        ca.step();
        // Even with periodic boundary, neighbors of center become alive
        assert!(ca.alive_count() >= 3);
    }
}
