use us_tax_brackets::{FilingStatus, TaxYear};

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

    fn standard_deduction(&self, status: FilingStatus) -> Usd {
        match status {
            FilingStatus::Single | FilingStatus::MarriedFilingSeparately => {
                Usd::from_dollars(15_750)
            }
            FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => {
                Usd::from_dollars(31_500)
            }
            FilingStatus::HeadOfHousehold => Usd::from_dollars(23_625),
        }
    }
}
