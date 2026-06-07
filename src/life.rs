//! Conway's Game of Life and 2D cellular automata.
//!
//! Implements a 2D grid with periodic (toroidal) boundary conditions.
//! Supports the standard B3/S23 rule (Conway's Game of Life) as well as
//! custom birth/survival rules.

/// A 2D cellular automaton supporting Conway's Game of Life and variants.
///
/// The grid uses periodic (toroidal) boundary conditions.
///
/// # Examples
///
/// ```
/// use cellular_automata_rs::life::GameOfLife;
///
/// let mut gol = GameOfLife::new(5, 5);
/// gol.set(2, 1, true);
/// gol.set(2, 2, true);
/// gol.set(2, 3, true);
/// gol.step(); // Blinker oscillates
/// ```
#[derive(Debug, Clone)]
pub struct GameOfLife {
    /// Grid width (number of columns).
    pub width: usize,
    /// Grid height (number of rows).
    pub height: usize,
    /// Cell states stored row-major: `state[y * width + x]`.
    pub state: Vec<bool>,
    /// Birth rule: dead cell becomes alive if neighbor count is in this set.
    pub birth: [bool; 9],
    /// Survival rule: alive cell stays alive if neighbor count is in this set.
    pub survival: [bool; 9],
}

impl GameOfLife {
    /// Create a new Game of Life grid with the given dimensions (standard B3/S23 rules).
    ///
    /// All cells start dead.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            state: vec![false; width * height],
            birth: Self::b3s23_birth(),
            survival: Self::b3s23_survival(),
        }
    }

    /// Create with custom B/S rules.
    ///
    /// `birth_rules` and `survival_rules` are slices of neighbor counts (0-8).
    pub fn new_with_rules(
        width: usize,
        height: usize,
        birth_rules: &[usize],
        survival_rules: &[usize],
    ) -> Self {
        let mut birth = [false; 9];
        let mut survival = [false; 9];
        for &b in birth_rules {
            assert!(b <= 8);
            birth[b] = true;
        }
        for &s in survival_rules {
            assert!(s <= 8);
            survival[s] = true;
        }
        Self {
            width,
            height,
            state: vec![false; width * height],
            birth,
            survival,
        }
    }

    fn b3s23_birth() -> [bool; 9] {
        let mut b = [false; 9];
        b[3] = true;
        b
    }

    fn b3s23_survival() -> [bool; 9] {
        let mut s = [false; 9];
        s[2] = true;
        s[3] = true;
        s
    }

    /// Get the cell state at (x, y) with periodic boundary.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> bool {
        let x = x % self.width;
        let y = y % self.height;
        self.state[y * self.width + x]
    }

    /// Set the cell state at (x, y).
    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width && y < self.height {
            self.state[y * self.width + x] = alive;
        }
    }

    /// Count alive neighbors (Moore neighborhood) for cell at (x, y).
    pub fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let w = self.width;
        let h = self.height;
        for dy in [h - 1, 0, 1] {
            for dx in [w - 1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.get((x + dx) % w, (y + dy) % h) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Advance the simulation by one generation.
    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;
        let mut next = vec![false; w * h];
        for y in 0..h {
            for x in 0..w {
                let n = self.count_neighbors(x, y);
                let alive = self.get(x, y);
                next[y * w + x] = if alive {
                    self.survival[n]
                } else {
                    self.birth[n]
                };
            }
        }
        self.state = next;
    }

    /// Advance by `n` generations.
    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Count total alive cells.
    pub fn alive_count(&self) -> usize {
        self.state.iter().filter(|&&b| b).count()
    }

    /// Check if all cells are dead.
    pub fn is_all_dead(&self) -> bool {
        !self.state.iter().any(|&b| b)
    }

    /// Render the grid as a string with `█` for alive and `·` for dead.
    pub fn to_string_visual(&self) -> String {
        let mut s = String::with_capacity(self.width * self.height + self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(if self.get(x, y) { '█' } else { '·' });
            }
            if y + 1 < self.height {
                s.push('\n');
            }
        }
        s
    }

    /// Load a pattern from a multi-line string where `#` or `O` means alive.
    ///
    /// Offsets the pattern to start at (dx, dy).
    pub fn load_pattern(&mut self, pattern: &str, dx: usize, dy: usize) {
        for (y, line) in pattern.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' || ch == 'O' {
                    self.set(dx + x, dy + y, true);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid_all_dead() {
        let gol = GameOfLife::new(5, 5);
        assert!(gol.is_all_dead());
        assert_eq!(gol.alive_count(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut gol = GameOfLife::new(5, 5);
        gol.set(2, 3, true);
        assert!(gol.get(2, 3));
        assert!(!gol.get(2, 2));
    }

    #[test]
    fn test_block_still_life() {
        // A 2x2 block is a still life — it doesn't change
        let mut gol = GameOfLife::new(5, 5);
        gol.set(1, 1, true);
        gol.set(2, 1, true);
        gol.set(1, 2, true);
        gol.set(2, 2, true);

        let state_before = gol.state.clone();
        gol.step();
        assert_eq!(gol.state, state_before, "Block should be a still life");
    }

    #[test]
    fn test_beehive_still_life() {
        // Beehive:
        //  .##.
        // #..#.
        // wait, let me do it correctly:
        // .##.
        // #..#
        // .##.
        let mut gol = GameOfLife::new(6, 6);
        gol.set(1, 0, true);
        gol.set(2, 0, true);
        gol.set(0, 1, true);
        gol.set(3, 1, true);
        gol.set(1, 2, true);
        gol.set(2, 2, true);

        let state_before = gol.state.clone();
        gol.step();
        assert_eq!(gol.state, state_before, "Beehive should be a still life");
    }

    #[test]
    fn test_blinker_oscillator() {
        // Blinker (period 2):
        // ###  -> vertical -> horizontal
        let mut gol = GameOfLife::new(5, 5);
        gol.set(1, 2, true);
        gol.set(2, 2, true);
        gol.set(3, 2, true);

        let initial = gol.state.clone();
        gol.step();
        // Should be vertical: (2,1), (2,2), (2,3)
        assert!(gol.get(2, 1));
        assert!(gol.get(2, 2));
        assert!(gol.get(2, 3));
        assert!(!gol.get(1, 2));
        assert!(!gol.get(3, 2));

        gol.step();
        // Should return to original
        assert_eq!(gol.state, initial, "Blinker should have period 2");
    }

    #[test]
    fn test_glider_movement() {
        // Glider:
        // .#.
        // ..#
        // ###
        let mut gol = GameOfLife::new(10, 10);
        gol.set(1, 0, true);
        gol.set(2, 1, true);
        gol.set(0, 2, true);
        gol.set(1, 2, true);
        gol.set(2, 2, true);

        let count0 = gol.alive_count();
        gol.step_n(4);
        let count4 = gol.alive_count();
        // Glider preserves cell count
        assert_eq!(count0, count4, "Glider should preserve cell count");
    }

    #[test]
    fn test_count_neighbors() {
        let mut gol = GameOfLife::new(5, 5);
        gol.set(0, 0, true);
        gol.set(1, 0, true);
        gol.set(0, 1, true);
        // Cell (1,1) has 3 neighbors
        assert_eq!(gol.count_neighbors(1, 1), 3);
        // Cell (2, 2) has 0 neighbors
        assert_eq!(gol.count_neighbors(2, 2), 0);
    }

    #[test]
    fn test_count_neighbors_periodic() {
        let mut gol = GameOfLife::new(3, 3);
        gol.set(0, 0, true);
        // Cell (1,1) neighbors: check wrapping
        // (0,0) is a neighbor of (1,1) — yes, diagonal
        assert_eq!(gol.count_neighbors(1, 1), 1);
        // Cell (2,2) should see (0,0) as neighbor through wrapping
        assert_eq!(gol.count_neighbors(2, 2), 1);
    }

    #[test]
    fn test_visual_string() {
        let mut gol = GameOfLife::new(3, 2);
        gol.set(1, 0, true);
        gol.set(0, 1, true);
        let visual = gol.to_string_visual();
        assert_eq!(visual, "·█·\n█··");
    }

    #[test]
    fn test_load_pattern() {
        let mut gol = GameOfLife::new(5, 5);
        gol.load_pattern("##\n##", 1, 1);
        assert!(gol.get(1, 1));
        assert!(gol.get(2, 1));
        assert!(gol.get(1, 2));
        assert!(gol.get(2, 2));
        assert_eq!(gol.alive_count(), 4);
    }

    #[test]
    fn test_custom_rules() {
        // HighLife: B36/S23 — replicator
        let mut gol = GameOfLife::new_with_rules(5, 5, &[3, 6], &[2, 3]);
        gol.set(2, 2, true);
        gol.step();
        // In standard GoL, single cell dies; in HighLife with B3 only, single cell also dies
        assert!(gol.is_all_dead());
    }

    #[test]
    fn test_empty_grid_stays_empty() {
        let mut gol = GameOfLife::new(10, 10);
        gol.step_n(5);
        assert!(gol.is_all_dead());
    }

    #[test]
    fn test_step_n_consistency() {
        let mut gol1 = GameOfLife::new(8, 8);
        gol1.set(3, 3, true);
        gol1.set(4, 3, true);
        gol1.set(3, 4, true);

        let mut gol2 = gol1.clone();
        gol1.step_n(3);
        gol2.step();
        gol2.step();
        gol2.step();
        assert_eq!(gol1.state, gol2.state);
    }

    #[test]
    fn test_loaf_still_life() {
        // Loaf:
        // .##.
        // #..#
        // .#.#
        // ..#.
        let mut gol = GameOfLife::new(6, 6);
        let loaf = ".##.\n#..#\n.#.#\n..#.";
        gol.load_pattern(loaf, 1, 1);
        let state_before = gol.state.clone();
        gol.step();
        assert_eq!(gol.state, state_before, "Loaf should be a still life");
    }
}
