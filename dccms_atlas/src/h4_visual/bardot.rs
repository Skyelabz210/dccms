use super::alphabet::{GlyphAlphabet, SemanticRole};

/// Maya base-20 bar-and-dot numeral.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BarDotNumeral(pub u8);

impl BarDotNumeral {
    /// Construct a validated Maya base-20 numeral.
    pub fn new(v: u8) -> Option<BarDotNumeral> {
        if v <= 19 {
            Some(BarDotNumeral(v))
        } else {
            None
        }
    }

    /// Number of five-value bars in this numeral.
    pub fn bar_count(&self) -> u8 {
        self.0 / 5
    }

    /// Number of one-value dots in this numeral.
    pub fn dot_count(&self) -> u8 {
        self.0 % 5
    }
}

impl GlyphAlphabet<6> for BarDotNumeral {
    fn ordinal(&self) -> u64 {
        self.0 as u64
    }

    fn semantic_role(&self) -> SemanticRole {
        SemanticRole::Content
    }
}

impl GlyphAlphabet<7> for BarDotNumeral {
    fn ordinal(&self) -> u64 {
        self.0 as u64
    }

    fn semantic_role(&self) -> SemanticRole {
        SemanticRole::Content
    }
}

impl GlyphAlphabet<8> for BarDotNumeral {
    fn ordinal(&self) -> u64 {
        self.0 as u64
    }

    fn semantic_role(&self) -> SemanticRole {
        SemanticRole::Content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dresden_codex::{cram_address, SAFE_BASIS};

    #[test]
    fn new_accepts_0_through_19() {
        for value in 0..=19 {
            assert_eq!(BarDotNumeral::new(value), Some(BarDotNumeral(value)));
        }

        assert_eq!(BarDotNumeral::new(20), None);
        assert_eq!(BarDotNumeral::new(255), None);
    }

    #[test]
    fn ordinal_matches_value() {
        for value in 0..=19 {
            let numeral = BarDotNumeral::new(value).unwrap();
            assert_eq!(<BarDotNumeral as GlyphAlphabet<6>>::ordinal(&numeral), value as u64);
        }
    }

    #[test]
    fn semantic_role_is_content() {
        for value in 0..=19 {
            let numeral = BarDotNumeral::new(value).unwrap();
            assert_eq!(
                <BarDotNumeral as GlyphAlphabet<6>>::semantic_role(&numeral),
                SemanticRole::Content
            );
        }
    }

    #[test]
    fn operator_consistency() {
        for value in 0..=19 {
            let numeral = BarDotNumeral::new(value).unwrap();
            assert_eq!(
                <BarDotNumeral as GlyphAlphabet<6>>::address(&numeral, &SAFE_BASIS),
                cram_address(<BarDotNumeral as GlyphAlphabet<6>>::ordinal(&numeral))
            );
        }
    }

    #[test]
    fn bars_and_dots_count() {
        for value in 0..=19 {
            let numeral = BarDotNumeral::new(value).unwrap();
            assert_eq!(numeral.bar_count(), value / 5);
            assert_eq!(numeral.dot_count(), value % 5);
        }
    }
}
