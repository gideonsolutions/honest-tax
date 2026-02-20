pub mod y2025;

use us_tax_brackets::{FilingStatus, TaxYear};

use crate::Usd;

/// Year-specific tax parameters consumed by [`crate::spine::compute_spine`].
///
/// Each tax year gets its own implementation that encodes the IRS-published
/// figures (standard deduction, etc.) for that filing season.
pub trait TaxYearRules {
    fn year(&self) -> TaxYear;
    fn standard_deduction(&self, status: FilingStatus) -> Usd;
}
