//! Cyclic cellular automata.
//!
//! In a cyclic CA, each cell has a state in `{0, 1, ..., n-1}`. A cell in state `k`
//! advances to state `(k+1) % n` if any of its neighbors is in state `(k+1) % n`.
//! Otherwise, the cell stays in its current state.
//!
//! This produces mesmerizing spiraling wave patterns.

/// A cyclic cellular automaton on a 2D grid.
///
/// # Examples
///
/// ```
/// use cellular_automata_rs::cyclic::CyclicCA;
///
/// let mut ca = CyclicCA::new(20, 20, 4);
/// ca.set(10, 10, 1);
/// ca.step();
/// ```
#[derive(Debug, Clone)]
pub struct CyclicCA {
    /// Grid width.
    pub width: usize,
    /// Grid height.
    pub height: usize,
    /// Number of states (colors).
    pub num_states: u8,
    /// Cell states in row-major order.
    pub grid: Vec<u8>,
    /// Neighborhood range (1 = Moore, can be larger).
    pub range: usize,
}

impl CyclicCA {
    /// Create a new cyclic CA with the given dimensions and number of states.
    ///
    /// All cells start in state 0.
    pub fn new(width: usize, height: usize, num_states: u8) -> Self {
        assert!(num_states >= 2, "Need at least 2 states");
        Self {
            width,
            height,
            num_states,
            grid: vec![0; width * height],
            range: 1,
        }
    }

    /// Create with a custom neighborhood range.
    ///
    /// A range of `r` means the neighborhood extends `r` cells in each direction
    /// (Chebyshev distance ≤ r).
    pub fn new_with_range(width: usize, height: usize, num_states: u8, range: usize) -> Self {
        assert!(num_states >= 2);
        Self {
            width,
            height,
            num_states,
            grid: vec![0; width * height],
            range: range.max(1),
        }
    }

    /// Get the cell state at (x, y) with periodic wrapping.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[(y % self.height) * self.width + (x % self.width)]
    }

    /// Set the cell state at (x, y).
    pub fn set(&mut self, x: usize, y: usize, state: u8) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = state % self.num_states;
        }
    }

    /// Fill the grid with random states using a simple seed-based generator.
    ///
    /// Uses a linear congruential generator for reproducibility.
    pub fn randomize_with_seed(&mut self, seed: u64) {
        let mut rng = seed;
        for cell in &mut self.grid {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            *cell = ((rng >> 33) as u8) % self.num_states;
        }
    }

    /// Advance the simulation by one generation.
    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;
        let n = self.num_states;
        let r = self.range;
        let mut next = self.grid.clone();

        for y in 0..h {
            for x in 0..w {
                let current = self.grid[y * w + x];
                let next_state = (current + 1) % n;

                // Check if any neighbor has the next state
                let mut found = false;
                for dy in 0..=(2 * r) {
                    for dx in 0..=(2 * r) {
                        if dx == r && dy == r {
                            continue;
                        }
                        let nx = (x + dx + w - r) % w;
                        let ny = (y + dy + h - r) % h;
                        if self.grid[ny * w + nx] == next_state {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        break;
                    }
                }

                if found {
                    next[y * w + x] = next_state;
                }
            }
        }

        self.grid = next;
    }

    /// Advance by `n` generations.
    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Count cells with a given state.
    pub fn count_state(&self, state: u8) -> usize {
        self.grid.iter().filter(|&&s| s == state).count()
    }

    /// Count the number of cells that changed state in the last step.
    /// (Useful for detecting convergence; not tracked internally.)
    pub fn diff_count(&self, prev: &[u8]) -> usize {
        self.grid
            .iter()
            .zip(prev.iter())
            .filter(|(a, b)| a != b)
            .count()
    }

    /// Render the grid as a string, using hex digits for states 0-15.
    pub fn to_string_visual(&self) -> String {
        let mut s = String::with_capacity(self.width * self.height + self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.grid[y * self.width + x];
                let ch = if v < 10 {
                    (b'0' + v) as char
                } else {
                    (b'A' + v - 10) as char
                };
                s.push(ch);
            }
            if y + 1 < self.height {
                s.push('\n');
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_all_zero() {
        let ca = CyclicCA::new(5, 5, 4);
        assert!(ca.grid.iter().all(|&s| s == 0));
    }

    #[test]
    fn test_set_get() {
        let mut ca = CyclicCA::new(5, 5, 4);
        ca.set(2, 3, 2);
        assert_eq!(ca.get(2, 3), 2);
    }

    #[test]
    fn test_set_wraps_state() {
        let mut ca = CyclicCA::new(5, 5, 4);
        ca.set(0, 0, 5); // 5 % 4 = 1
        assert_eq!(ca.get(0, 0), 1);
    }

    #[test]
    fn test_single_seed_propagates() {
        let mut ca = CyclicCA::new(5, 5, 3);
        ca.set(2, 2, 1); // Seed with state 1
        ca.step();
        // Cell (2,2) was state 1, its successor is 2. It should check if any neighbor has state 2.
        // No neighbor has state 2, so (2,2) stays at 1.
        // State-0 neighbors of (2,2): their successor is 1. Cell (2,2) has state 1 = successor.
        // So state-0 neighbors of the seed should advance to 1.
        assert_eq!(ca.get(2, 2), 1, "Seed should stay (no neighbor with state 2)");
        assert_eq!(ca.get(1, 2), 1, "Neighbor of seed should advance to 1");
        assert_eq!(ca.get(3, 2), 1, "Neighbor of seed should advance to 1");
    }

    #[test]
    fn test_no_change_without_successor_neighbor() {
        let mut ca = CyclicCA::new(3, 3, 4);
        ca.set(0, 0, 0);
        ca.set(1, 0, 0);
        ca.set(2, 0, 0);
        ca.set(0, 1, 0);
        ca.set(1, 1, 2); // 2 is not successor of 0
        ca.set(2, 1, 0);
        ca.set(0, 2, 0);
        ca.set(1, 2, 0);
        ca.set(2, 2, 0);
        let prev = ca.grid.clone();
        ca.step();
        // All state-0 cells: successor is 1, no neighbor has state 1
        assert_eq!(ca.grid, prev, "No cell should change without successor neighbor");
    }

    #[test]
    fn test_periodic_wrapping() {
        let mut ca = CyclicCA::new(3, 3, 3);
        ca.set(0, 0, 1);
        // Cell (2,0) is a neighbor of (0,0) through wrapping? No: (2,0) neighbors are (1,0), (0,0), (2,2), (1,2), (2,1), (1,1), (0,1), (0,2)
        // Actually (2,0) neighbors (0,0) through wrapping since (2+1)%3=0
        ca.step();
        assert_eq!(ca.get(2, 0), 1, "State should propagate through periodic boundary");
    }

    #[test]
    fn test_randomize_with_seed() {
        let mut ca1 = CyclicCA::new(10, 10, 4);
        ca1.randomize_with_seed(42);

        let mut ca2 = CyclicCA::new(10, 10, 4);
        ca2.randomize_with_seed(42);

        assert_eq!(ca1.grid, ca2.grid, "Same seed should produce same grid");
        // Should have non-zero states
        assert!(ca1.grid.iter().any(|&s| s > 0));
    }

    #[test]
    fn test_different_seeds_different_grids() {
        let mut ca1 = CyclicCA::new(10, 10, 4);
        ca1.randomize_with_seed(42);

        let mut ca2 = CyclicCA::new(10, 10, 4);
        ca2.randomize_with_seed(99);

        assert_ne!(ca1.grid, ca2.grid, "Different seeds should differ");
    }

    #[test]
    fn test_step_n_consistency() {
        let mut ca1 = CyclicCA::new(5, 5, 3);
        ca1.randomize_with_seed(42);
        ca1.step_n(3);

        let mut ca2 = CyclicCA::new(5, 5, 3);
        ca2.randomize_with_seed(42);
        ca2.step();
        ca2.step();
        ca2.step();

        assert_eq!(ca1.grid, ca2.grid);
    }

    #[test]
    fn test_count_state() {
        let mut ca = CyclicCA::new(3, 3, 2);
        ca.set(0, 0, 1);
        assert_eq!(ca.count_state(0), 8);
        assert_eq!(ca.count_state(1), 1);
    }

    #[test]
    fn test_diff_count() {
        let mut ca = CyclicCA::new(3, 3, 3);
        let prev = ca.grid.clone();
        ca.set(1, 1, 1);
        assert_eq!(ca.diff_count(&prev), 1);
    }

    #[test]
    fn test_visual_string() {
        let mut ca = CyclicCA::new(3, 1, 4);
        ca.set(0, 0, 0);
        ca.set(1, 0, 2);
        ca.set(2, 0, 3);
        assert_eq!(ca.to_string_visual(), "023");
    }

    #[test]
    fn test_cycle_completeness() {
        let mut ca = CyclicCA::new(5, 5, 2);
        ca.randomize_with_seed(7);
        ca.step_n(10);
        // After many steps, grid should still have valid states
        for &s in &ca.grid {
            assert!(s < 2, "State should be valid");
        }
    }

    #[test]
    #[should_panic(expected = "Need at least 2 states")]
    fn test_too_few_states() {
        CyclicCA::new(5, 5, 1);
    }

    #[test]
    fn test_range_greater_than_1() {
        let mut ca = CyclicCA::new_with_range(10, 10, 3, 2);
        ca.randomize_with_seed(42);
        ca.step();
        // Should not crash, grid should still be valid
        for &s in &ca.grid {
            assert!(s < 3);
        }
    }

    #[test]
    fn test_clone_independence() {
        let mut ca = CyclicCA::new(5, 5, 3);
        ca.randomize_with_seed(42);
        let clone = ca.clone();
        ca.step();
        assert_ne!(ca.grid, clone.grid, "Clone should be independent");
    }
}
