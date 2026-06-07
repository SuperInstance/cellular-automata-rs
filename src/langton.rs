//! Langton's Ant and multi-ant variants.
//!
//! Langton's Ant is a 2D Turing machine on a grid of black/white cells.
//! The ant moves according to:
//! - On a white cell: turn right, flip color, move forward
//! - On a black cell: turn left, flip color, move forward
//!
//! After ~10,000 steps from a blank grid, the ant builds a "highway" pattern.

/// Cardinal directions for ant movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Turn 90° clockwise (right).
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Turn 90° counter-clockwise (left).
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    /// Get the displacement (dx, dy) for moving forward one step.
    pub fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

/// A single ant with position and direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ant {
    /// Current x position.
    pub x: isize,
    /// Current y position.
    pub y: isize,
    /// Current facing direction.
    pub direction: Direction,
}

impl Ant {
    /// Create a new ant at the given position facing North.
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
            direction: Direction::North,
        }
    }

    /// Create an ant at the given position with a specific direction.
    pub fn with_direction(x: isize, y: isize, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    /// Move the ant forward one step in its current direction.
    pub fn move_forward(&mut self) {
        let (dx, dy) = self.direction.delta();
        self.x += dx;
        self.y += dy;
    }
}

/// Langton's Ant simulation on a bounded grid with wrapping.
///
/// Cell states are `u8` values. For the classic Langton's Ant, state 0 = white
/// and state 1 = black. Multi-color variants use more states.
///
/// # Examples
///
/// ```
/// use cellular_automata_rs::langton::LangtonsAnt;
///
/// let mut ant = LangtonsAnt::new(50, 50);
/// ant.step_n(100);
/// assert!(ant.step_count() > 0);
/// ```
#[derive(Debug, Clone)]
pub struct LangtonsAnt {
    /// Grid width.
    pub width: usize,
    /// Grid height.
    pub height: usize,
    /// Cell states (row-major).
    pub grid: Vec<u8>,
    /// The ant.
    pub ant: Ant,
    /// Number of steps taken.
    steps: usize,
    /// Number of colors (states per cell). Default 2 for classic Langton's Ant.
    pub num_colors: u8,
    /// Turn rule for each color: `true` = turn right, `false` = turn left.
    /// For classic 2-color: [true, false] (RL).
    pub turn_rules: Vec<bool>,
}

impl LangtonsAnt {
    /// Create a classic Langton's Ant (RL rule) on a grid of the given size.
    ///
    /// The ant starts at the center facing North. All cells start white (state 0).
    pub fn new(width: usize, height: usize) -> Self {
        let cx = (width / 2) as isize;
        let cy = (height / 2) as isize;
        Self {
            width,
            height,
            grid: vec![0; width * height],
            ant: Ant::new(cx, cy),
            steps: 0,
            num_colors: 2,
            turn_rules: vec![true, false], // R, L (classic)
        }
    }

    /// Create a multi-color Langton's Ant variant.
    ///
    /// `turn_rules` is a string like "RLR" where 'R' = turn right, 'L' = turn left.
    /// The number of characters determines the number of cell states.
    pub fn new_with_rules(width: usize, height: usize, turn_rules: &str) -> Self {
        let cx = (width / 2) as isize;
        let cy = (height / 2) as isize;
        let rules: Vec<bool> = turn_rules
            .chars()
            .map(|c| c == 'R' || c == 'r')
            .collect();
        Self {
            width,
            height,
            grid: vec![0; width * height],
            ant: Ant::new(cx, cy),
            steps: 0,
            num_colors: rules.len() as u8,
            turn_rules: rules,
        }
    }

    /// Get the cell state at (x, y) with periodic wrapping.
    pub fn get(&self, x: isize, y: isize) -> u8 {
        let x = ((x % self.width as isize) + self.width as isize) as usize % self.width;
        let y = ((y % self.height as isize) + self.height as isize) as usize % self.height;
        self.grid[y * self.width + x]
    }

    /// Set the cell state at (x, y) with periodic wrapping.
    pub fn set(&mut self, x: isize, y: isize, state: u8) {
        let x = ((x % self.width as isize) + self.width as isize) as usize % self.width;
        let y = ((y % self.height as isize) + self.height as isize) as usize % self.height;
        self.grid[y * self.width + x] = state;
    }

    /// Advance the simulation by one step.
    pub fn step(&mut self) {
        let color = self.get(self.ant.x, self.ant.y);
        let rule_idx = color as usize % self.turn_rules.len();

        // Turn based on current cell color
        if self.turn_rules[rule_idx] {
            self.ant.direction = self.ant.direction.turn_right();
        } else {
            self.ant.direction = self.ant.direction.turn_left();
        }

        // Increment cell color
        let new_color = (color + 1) % self.num_colors;
        self.set(self.ant.x, self.ant.y, new_color);

        // Move forward
        self.ant.move_forward();
        self.steps += 1;
    }

    /// Advance by `n` steps.
    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Get the number of steps taken.
    pub fn step_count(&self) -> usize {
        self.steps
    }

    /// Count cells with the given state.
    pub fn count_state(&self, state: u8) -> usize {
        self.grid.iter().filter(|&&s| s == state).count()
    }

    /// Render the grid as a string (0='·', 1='█', 2-9='0'-'7').
    pub fn to_string_visual(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let ch = match self.grid[y * self.width + x] {
                    0 => '·',
                    1 => '█',
                    n => (b'0' + n - 2) as char,
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
    fn test_direction_turns() {
        assert_eq!(Direction::North.turn_right(), Direction::East);
        assert_eq!(Direction::East.turn_right(), Direction::South);
        assert_eq!(Direction::South.turn_right(), Direction::West);
        assert_eq!(Direction::West.turn_right(), Direction::North);

        assert_eq!(Direction::North.turn_left(), Direction::West);
        assert_eq!(Direction::West.turn_left(), Direction::South);
        assert_eq!(Direction::South.turn_left(), Direction::East);
        assert_eq!(Direction::East.turn_left(), Direction::North);
    }

    #[test]
    fn test_direction_full_rotation() {
        let d = Direction::North;
        let d = d.turn_right().turn_right().turn_right().turn_right();
        assert_eq!(d, Direction::North);
    }

    #[test]
    fn test_direction_deltas() {
        assert_eq!(Direction::North.delta(), (0, -1));
        assert_eq!(Direction::East.delta(), (1, 0));
        assert_eq!(Direction::South.delta(), (0, 1));
        assert_eq!(Direction::West.delta(), (-1, 0));
    }

    #[test]
    fn test_ant_new() {
        let ant = Ant::new(5, 5);
        assert_eq!(ant.x, 5);
        assert_eq!(ant.y, 5);
        assert_eq!(ant.direction, Direction::North);
    }

    #[test]
    fn test_ant_move_forward() {
        let mut ant = Ant::new(5, 5);
        ant.move_forward(); // North
        assert_eq!(ant.x, 5);
        assert_eq!(ant.y, 4);
    }

    #[test]
    fn test_ant_turn_and_move() {
        let mut ant = Ant::new(5, 5);
        ant.direction = ant.direction.turn_right(); // Now East
        ant.move_forward();
        assert_eq!(ant.x, 6);
        assert_eq!(ant.y, 5);
    }

    #[test]
    fn test_langton_initial_state() {
        let ant = LangtonsAnt::new(10, 10);
        assert_eq!(ant.step_count(), 0);
        assert_eq!(ant.ant.x, 5);
        assert_eq!(ant.ant.y, 5);
        assert!(ant.grid.iter().all(|&s| s == 0));
    }

    #[test]
    fn test_langton_single_step() {
        let mut la = LangtonsAnt::new(10, 10);
        la.step();
        assert_eq!(la.step_count(), 1);
        // Started on white (0) -> turn right (now East), flip to 1, move forward
        assert_eq!(la.ant.direction, Direction::East);
        assert_eq!(la.ant.x, 6);
        assert_eq!(la.ant.y, 5);
        // Cell (5,5) should now be state 1
        assert_eq!(la.get(5, 5), 1);
    }

    #[test]
    fn test_langton_two_steps() {
        let mut la = LangtonsAnt::new(10, 10);
        la.step(); // On white -> R, flip, move to (6,5), facing East
        la.step(); // On white -> R, flip, move to (6,6), facing South
        assert_eq!(la.ant.direction, Direction::South);
        assert_eq!(la.ant.x, 6);
        assert_eq!(la.ant.y, 6);
        assert_eq!(la.get(5, 5), 1);
        assert_eq!(la.get(6, 5), 1);
    }

    #[test]
    fn test_langton_highway_emerges() {
        // After ~10,000 steps, the classic ant builds a highway
        let mut la = LangtonsAnt::new(100, 100);
        la.step_n(11000);
        // The ant should have moved significantly from center
        let dx = (la.ant.x - 50).abs();
        let dy = (la.ant.y - 50).abs();
        assert!(dx > 5 || dy > 5, "Ant should have moved significantly after highway formation");
    }

    #[test]
    fn test_langton_step_n() {
        let mut la1 = LangtonsAnt::new(10, 10);
        la1.step_n(5);

        let mut la2 = LangtonsAnt::new(10, 10);
        for _ in 0..5 {
            la2.step();
        }
        assert_eq!(la1.grid, la2.grid);
        assert_eq!(la1.ant.x, la2.ant.x);
        assert_eq!(la1.ant.y, la2.ant.y);
    }

    #[test]
    fn test_multi_color_variant() {
        let mut la = LangtonsAnt::new_with_rules(20, 20, "RLR");
        assert_eq!(la.num_colors, 3);
        assert_eq!(la.turn_rules.len(), 3);
        la.step_n(100);
        // Should not crash; some cells should be non-zero
        assert!(la.count_state(0) < la.width * la.height);
    }

    #[test]
    fn test_periodic_wrapping() {
        let mut la = LangtonsAnt::new(3, 3);
        la.ant.x = 0;
        la.ant.y = 0;
        la.ant.direction = Direction::West;
        la.step(); // On white (0): turn right (West->North), flip, move North
        // Now facing North. Move forward: (0, -1). With wrapping: y = (0-1) % 3
        // ant.y should wrap. Let's compute: -1 % 3 = -1 in Rust, but the step uses move_forward
        // which just does y += -1 = -1
        // The ant position is isize, so it can go negative. But get/set do wrapping.
        // So ant.y will be -1 as isize.
        assert_eq!(la.ant.y, -1);
        assert_eq!(la.ant.direction, Direction::North);
    }

    #[test]
    fn test_count_state() {
        let mut la = LangtonsAnt::new(5, 5);
        la.step_n(10);
        assert_eq!(la.count_state(0) + la.count_state(1), 25);
    }

    #[test]
    fn test_visual_string() {
        let la = LangtonsAnt::new(3, 3);
        let v = la.to_string_visual();
        assert!(v.contains('·'));
    }
}
