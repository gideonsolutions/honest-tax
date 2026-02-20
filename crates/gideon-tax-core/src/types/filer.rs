/// A filer (taxpayer or spouse) for purposes of the additional standard
/// deduction and other age/blindness rules.
#[derive(Debug, Clone, Copy, Default)]
pub struct Filer {
    pub is_65_or_older: bool,
    pub is_blind: bool,
}

impl Filer {
    /// Number of checked boxes (0, 1, or 2) that qualify for the additional
    /// standard deduction.
    pub fn checked_boxes(self) -> i64 {
        self.is_65_or_older as i64 + self.is_blind as i64
    }
}
