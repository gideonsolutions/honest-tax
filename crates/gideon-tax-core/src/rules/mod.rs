pub mod y2025;

use us_tax_brackets::{FilingStatus, TaxYear};

use crate::Usd;
use crate::types::Filer;

/// Year-specific tax parameters consumed by [`crate::spine::compute_spine`].
///
/// Each tax year gets its own implementation that supplies the IRS-published
/// dollar amounts. The [`standard_deduction`](TaxYearRules::standard_deduction)
/// algorithm is a provided method that combines them.
pub trait TaxYearRules {
    fn year(&self) -> TaxYear;

    /// Base standard deduction for Single or MFS filers.
    fn single_mfs_typical_standard_deduction(&self) -> Usd;

    /// Base standard deduction for MFJ or QSS filers.
    fn mfj_qss_typical_standard_deduction(&self) -> Usd;

    /// Base standard deduction for Head of Household filers.
    fn hoh_typical_standard_deduction(&self) -> Usd;

    /// Per-box addition for Single or Head of Household filers.
    fn additional_deduction_unmarried(&self) -> Usd;

    /// Per-box addition for MFJ, MFS, or QSS filers.
    fn additional_deduction_married(&self) -> Usd;

    /// Amount added to a dependent's earned income before clamping.
    fn dependent_earned_income_addition(&self) -> Usd;

    /// Minimum standard deduction for a dependent filer.
    fn dependent_minimum_deduction(&self) -> Usd;

    /// Base standard deduction before any age/blindness additions.
    fn typical_standard_deduction(&self, status: FilingStatus) -> Usd {
        use FilingStatus::*;
        match status {
            Single | MarriedFilingSeparately => self.single_mfs_typical_standard_deduction(),
            MarriedFilingJointly | QualifyingSurvivingSpouse => {
                self.mfj_qss_typical_standard_deduction()
            }
            HeadOfHousehold => self.hoh_typical_standard_deduction(),
        }
    }

    /// Computes the full standard deduction from year-specific constants.
    ///
    /// Returns $0 for dual-status aliens or MFS when spouse itemizes.
    /// Otherwise applies the dependent formula or the typical base, plus
    /// an additional amount per qualifying age/blindness box.
    fn standard_deduction(&self, params: &DeductionParams) -> Usd {
        use FilingStatus::*;

        // ── Zero-deduction overrides ────────────────────────────────
        if params.is_dual_status_alien {
            return Usd::ZERO;
        }
        if params.filing_status == MarriedFilingSeparately && params.spouse_itemizes {
            return Usd::ZERO;
        }

        // ── Base amount ─────────────────────────────────────────────
        let base = self.typical_standard_deduction(params.filing_status);

        // ── Additional amount per qualifying box ────────────────────
        let per_box = match params.filing_status {
            Single | HeadOfHousehold => self.additional_deduction_unmarried(),
            _ => self.additional_deduction_married(),
        };

        let boxes = match params.filing_status {
            Single | HeadOfHousehold => params.taxpayer.checked_boxes(),
            MarriedFilingJointly | MarriedFilingSeparately | QualifyingSurvivingSpouse => {
                params.taxpayer.checked_boxes() + params.spouse.map_or(0, |s| s.checked_boxes())
            }
        };

        let additional = per_box * boxes;

        // ── Dependent vs. non-dependent base ────────────────────────
        if params.is_dependent {
            let earned_plus = params.earned_income + self.dependent_earned_income_addition();
            let floor = self.dependent_minimum_deduction();
            let capped_base = earned_plus.max(floor).min(base);
            capped_base + additional
        } else {
            base + additional
        }
    }
}

/// Input to [`TaxYearRules::standard_deduction`].
///
/// When `filing_status` is [`FilingStatus::MarriedFilingSeparately`], a
/// `spouse` may only be provided if the spouse had no income, is not filing
/// a return, and cannot be claimed as a dependent on another person's return.
pub struct DeductionParams {
    pub filing_status: FilingStatus,
    pub taxpayer: Filer,
    pub spouse: Option<Filer>,
    /// `true` if the taxpayer can be claimed as a dependent on another
    /// person's return, **or** if filing jointly and the spouse can be.
    pub is_dependent: bool,
    pub is_dual_status_alien: bool,
    pub spouse_itemizes: bool,
    pub earned_income: Usd,
}
