use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::rules::TaxYearRules;

/// IRS-published parameters for tax year 2025 (filed in 2026).
///
/// See: <https://www.irs.gov/instructions/i1040gi#en_US_2025_publink1000158207>
pub struct Rules2025;

impl TaxYearRules for Rules2025 {
    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn single_mfs_typical_standard_deduction(&self) -> Usd {
        Usd::from_dollars(15_750)
    }

    fn mfj_qss_typical_standard_deduction(&self) -> Usd {
        Usd::from_dollars(31_500)
    }

    fn hoh_typical_standard_deduction(&self) -> Usd {
        Usd::from_dollars(23_625)
    }

    fn additional_deduction_unmarried(&self) -> Usd {
        Usd::from_dollars(2_000)
    }

    fn additional_deduction_married(&self) -> Usd {
        Usd::from_dollars(1_600)
    }

    fn dependent_earned_income_addition(&self) -> Usd {
        Usd::from_dollars(450)
    }

    fn dependent_minimum_deduction(&self) -> Usd {
        Usd::from_dollars(1_350)
    }
}

#[cfg(test)]
mod tests {
    use us_tax_brackets::FilingStatus;

    use super::*;
    use crate::rules::DeductionParams;
    use crate::types::Filer;

    const BLIND: Filer = Filer {
        is_65_or_older: false,
        is_blind: true,
    };
    const SENIOR: Filer = Filer {
        is_65_or_older: true,
        is_blind: false,
    };
    const SENIOR_BLIND: Filer = Filer {
        is_65_or_older: true,
        is_blind: true,
    };

    fn params(status: FilingStatus) -> DeductionParams {
        DeductionParams {
            filing_status: status,
            taxpayer: Filer::default(),
            spouse: None,
            is_dependent: false,
            is_dual_status_alien: false,
            spouse_itemizes: false,
            earned_income: Usd::ZERO,
        }
    }

    // ── Non-dependent, no boxes ─────────────────────────────────────

    #[test]
    fn base_single() {
        let d = Rules2025.standard_deduction(&params(FilingStatus::Single));
        assert_eq!(d, Usd::from_dollars(15_750));
    }

    #[test]
    fn base_mfj() {
        let d = Rules2025.standard_deduction(&params(FilingStatus::MarriedFilingJointly));
        assert_eq!(d, Usd::from_dollars(31_500));
    }

    #[test]
    fn base_hoh() {
        let d = Rules2025.standard_deduction(&params(FilingStatus::HeadOfHousehold));
        assert_eq!(d, Usd::from_dollars(23_625));
    }

    #[test]
    fn base_mfs() {
        let d = Rules2025.standard_deduction(&params(FilingStatus::MarriedFilingSeparately));
        assert_eq!(d, Usd::from_dollars(15_750));
    }

    #[test]
    fn base_qss() {
        let d = Rules2025.standard_deduction(&params(FilingStatus::QualifyingSurvivingSpouse));
        assert_eq!(d, Usd::from_dollars(31_500));
    }

    // ── Non-dependent, with boxes ───────────────────────────────────

    #[test]
    fn single_senior() {
        let mut p = params(FilingStatus::Single);
        p.taxpayer = SENIOR;
        // 15,750 + 1 * 2,000 = 17,750
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(17_750));
    }

    #[test]
    fn single_senior_blind() {
        let mut p = params(FilingStatus::Single);
        p.taxpayer = SENIOR_BLIND;
        // 15,750 + 2 * 2,000 = 19,750
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(19_750));
    }

    #[test]
    fn mfj_both_senior() {
        let mut p = params(FilingStatus::MarriedFilingJointly);
        p.taxpayer = SENIOR;
        p.spouse = Some(SENIOR);
        // 31,500 + 2 * 1,600 = 34,700
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(34_700));
    }

    #[test]
    fn mfj_all_four_boxes() {
        let mut p = params(FilingStatus::MarriedFilingJointly);
        p.taxpayer = SENIOR_BLIND;
        p.spouse = Some(SENIOR_BLIND);
        // 31,500 + 4 * 1,600 = 37,900
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(37_900));
    }

    #[test]
    fn hoh_blind() {
        let mut p = params(FilingStatus::HeadOfHousehold);
        p.taxpayer = BLIND;
        // 23,625 + 1 * 2,000 = 25,625
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(25_625));
    }

    // ── Dependent: earned income formula ────────────────────────────

    #[test]
    fn dependent_zero_earned_income_hits_floor() {
        let mut p = params(FilingStatus::Single);
        p.is_dependent = true;
        p.earned_income = Usd::ZERO;
        // max(0 + 450, 1,350) = 1,350; min(1,350, 15,750) = 1,350
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(1_350));
    }

    #[test]
    fn dependent_low_earned_income_hits_floor() {
        let mut p = params(FilingStatus::Single);
        p.is_dependent = true;
        p.earned_income = Usd::from_dollars(500);
        // max(500 + 450, 1,350) = 1,350; min(1,350, 15,750) = 1,350
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(1_350));
    }

    #[test]
    fn dependent_mid_earned_income_uses_formula() {
        let mut p = params(FilingStatus::Single);
        p.is_dependent = true;
        p.earned_income = Usd::from_dollars(5_000);
        // max(5,000 + 450, 1,350) = 5,450; min(5,450, 15,750) = 5,450
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(5_450));
    }

    #[test]
    fn dependent_high_earned_income_capped_at_base() {
        let mut p = params(FilingStatus::Single);
        p.is_dependent = true;
        p.earned_income = Usd::from_dollars(20_000);
        // max(20,000 + 450, 1,350) = 20,450; min(20,450, 15,750) = 15,750
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(15_750));
    }

    #[test]
    fn dependent_with_boxes() {
        let mut p = params(FilingStatus::Single);
        p.is_dependent = true;
        p.taxpayer = SENIOR_BLIND;
        p.earned_income = Usd::from_dollars(3_000);
        // base portion: max(3,000 + 450, 1,350) = 3,450; min(3,450, 15,750) = 3,450
        // additional: 2 * 2,000 = 4,000
        // total: 3,450 + 4,000 = 7,450
        assert_eq!(Rules2025.standard_deduction(&p), Usd::from_dollars(7_450));
    }

    // ── Zero-deduction overrides ────────────────────────────────────

    #[test]
    fn dual_status_alien_is_zero() {
        let mut p = params(FilingStatus::Single);
        p.is_dual_status_alien = true;
        p.taxpayer = SENIOR; // boxes don't matter
        assert_eq!(Rules2025.standard_deduction(&p), Usd::ZERO);
    }

    #[test]
    fn mfs_spouse_itemizes_is_zero() {
        let mut p = params(FilingStatus::MarriedFilingSeparately);
        p.spouse_itemizes = true;
        assert_eq!(Rules2025.standard_deduction(&p), Usd::ZERO);
    }
}
