//! Rule parsing and bit manipulation utilities for cellular automata.
//!
//! Wolfram rules are numbered 0-255, corresponding to the 8-bit lookup table
//! mapping a 3-cell neighborhood to the next state. Bit `i` of the rule number
//! gives the output for neighborhood pattern `i` (where `i = 4*left + 2*center + 1*right`).

/// A parsed Wolfram rule with its 8-bit lookup table.
///
/// Each of the 8 possible 3-cell neighborhoods maps to an output bit.
/// The rule number is the 8-bit encoding of these outputs.
///
/// # Examples
///
/// ```
/// use cellular_automata_rs::rule::Rule;
///
/// let r110 = Rule::new(110);
/// assert_eq!(r110.apply(false, true, true), true); // pattern 011 = 3, bit 3 of 110
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule {
    /// The Wolfram rule number (0-255).
    pub number: u8,
}

impl Rule {
    /// Create a new rule from a Wolfram rule number.
    ///
    /// # Panics
    ///
    /// Panics if the number is > 255 (it fits in u8, so this can't happen at compile time).
    #[inline]
    pub fn new(number: u8) -> Self {
        Self { number }
    }

    /// Apply the rule to a 3-cell neighborhood (left, center, right).
    ///
    /// The neighborhood is interpreted as a 3-bit number:
    /// `pattern = 4*left + 2*center + right`, and bit `pattern` of the rule number
    /// gives the output cell state.
    #[inline]
    pub fn apply(&self, left: bool, center: bool, right: bool) -> bool {
        let pattern = (left as u8) << 2 | (center as u8) << 1 | right as u8;
        (self.number >> pattern) & 1 == 1
    }

    /// Get the full 8-bit lookup table as an array indexed by pattern.
    ///
    /// Index 0 corresponds to neighborhood `000`, index 7 to `111`.
    pub fn lookup_table(&self) -> [bool; 8] {
        let mut table = [false; 8];
        for (i, slot) in table.iter_mut().enumerate() {
            *slot = (self.number >> i) & 1 == 1;
        }
        table
    }

    /// Check whether the rule is a "left shift" rule (Rule 170).
    ///
    /// A left shift maps each cell to its right neighbor.
    pub fn is_left_shift(&self) -> bool {
        self.number == 170
    }

    /// Check whether the rule is a "right shift" rule (Rule 240).
    ///
    /// A right shift maps each cell to its left neighbor.
    pub fn is_right_shift(&self) -> bool {
        self.number == 240
    }

    /// Classify the rule into Wolfram's four classes:
    /// - Class I: evolves to homogeneous state (e.g., Rule 0)
    /// - Class II: evolves to simple periodic structures
    /// - Class III: evolves to chaotic, aperiodic patterns (e.g., Rule 30)
    /// - Class IV: evolves to complex, localized structures (e.g., Rule 110)
    ///
    /// Note: classification is based on well-known results, not runtime analysis.
    pub fn wolfram_class(&self) -> &'static str {
        match self.number {
            0 | 8 | 32 | 40 | 128 | 136 | 160 | 168 => "I",
            1 | 2 | 3 | 4 | 5 | 6 | 7 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => "II",
            30 | 45 | 73 | 75 | 86 | 89 | 101 | 105 | 109 | 126 | 135 | 149 | 151
            | 153 | 161 | 165 | 167 | 169 | 181 | 182 | 183 | 195 | 210 | 225 => "III",
            110 | 54 | 62 | 147 => "IV",
            _ => "II/III",
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self::new(110)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_0_all_dead() {
        let rule = Rule::new(0);
        for l in [false, true] {
            for c in [false, true] {
                for r in [false, true] {
                    assert!(!rule.apply(l, c, r), "Rule 0 should always produce dead");
                }
            }
        }
    }

    #[test]
    fn test_rule_255_all_alive() {
        let rule = Rule::new(255);
        for l in [false, true] {
            for c in [false, true] {
                for r in [false, true] {
                    assert!(rule.apply(l, c, r), "Rule 255 should always produce alive");
                }
            }
        }
    }

    #[test]
    fn test_rule_110_known_pattern() {
        let rule = Rule::new(110);
        // Rule 110 lookup: 01101110 in binary
        // pattern 0 (000) -> 0
        assert!(!rule.apply(false, false, false));
        // pattern 1 (001) -> 1
        assert!(rule.apply(false, false, true));
        // pattern 2 (010) -> 1
        assert!(rule.apply(false, true, false));
        // pattern 3 (011) -> 1
        assert!(rule.apply(false, true, true));
        // pattern 4 (100) -> 0
        assert!(!rule.apply(true, false, false));
        // pattern 5 (101) -> 1
        assert!(rule.apply(true, false, true));
        // pattern 6 (110) -> 1
        assert!(rule.apply(true, true, false));
        // pattern 7 (111) -> 0
        assert!(!rule.apply(true, true, true));
    }

    #[test]
    fn test_rule_30_known_pattern() {
        let rule = Rule::new(30);
        // Rule 30 = 00011110
        // pattern 0 -> 0, pattern 1 -> 1, pattern 2 -> 1, pattern 3 -> 1
        // pattern 4 -> 1, pattern 5 -> 0, pattern 6 -> 0, pattern 7 -> 0
        assert!(!rule.apply(false, false, false));
        assert!(rule.apply(false, false, true));
        assert!(rule.apply(false, true, false));
        assert!(rule.apply(false, true, true));
        assert!(rule.apply(true, false, false));
        assert!(!rule.apply(true, false, true));
        assert!(!rule.apply(true, true, false));
        assert!(!rule.apply(true, true, true));
    }

    #[test]
    fn test_lookup_table_consistency() {
        let rule = Rule::new(110);
        let table = rule.lookup_table();
        for l in 0..2 {
            for c in 0..2 {
                for r in 0..2 {
                    let pattern = l * 4 + c * 2 + r;
                    assert_eq!(
                        rule.apply(l == 1, c == 1, r == 1),
                        table[pattern],
                        "Mismatch for pattern {pattern}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_left_shift_rule() {
        let rule = Rule::new(170);
        assert!(rule.is_left_shift());
        assert!(!rule.is_right_shift());
        // 170 = 10101010: output = right neighbor
        assert!(!rule.apply(false, false, false));
        assert!(rule.apply(false, false, true));
    }

    #[test]
    fn test_right_shift_rule() {
        let rule = Rule::new(240);
        assert!(rule.is_right_shift());
        assert!(!rule.is_left_shift());
        // 240 = 11110000: output = left neighbor
        assert!(!rule.apply(false, false, false));
        assert!(!rule.apply(false, false, true));
        assert!(rule.apply(true, false, false));
    }

    #[test]
    fn test_wolfram_classifications() {
        assert_eq!(Rule::new(0).wolfram_class(), "I");
        assert_eq!(Rule::new(30).wolfram_class(), "III");
        assert_eq!(Rule::new(110).wolfram_class(), "IV");
    }

    #[test]
    fn test_default_rule() {
        let rule = Rule::default();
        assert_eq!(rule.number, 110);
    }
}
