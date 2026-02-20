use std::collections::BTreeMap;
use std::fmt;

use us_tax_brackets::{self, FilingStatus, TaxYear};

use crate::Usd;
use crate::rules::TaxYearRules;

// ---------------------------------------------------------------------------
// Ledger keys
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    TotalIncome,
    Adjustments,
    AGI,
    Deductions,
    TaxableIncome,
    RegularTax,
    AdditionalTax,
    TotalTaxPreCredits,
    NonRefundableCredits,
    TaxAfterNonRefundableCredits,
    RefundableCredits,
    TotalTax,
    Withholding,
    EstimatedPayments,
    TotalPayments,
    Refund,
    AmountOwed,
}

pub type Ledger = BTreeMap<Key, Usd>;

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

pub struct ReturnInput {
    pub tax_year: TaxYear,
    pub filing_status: FilingStatus,
    pub w2_wages: Usd,
    pub fed_withholding: Usd,
}

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum SpineError {
    YearMismatch { input: TaxYear, rules: TaxYear },
    TaxComputeError(us_tax_brackets::TaxError),
}

impl From<us_tax_brackets::TaxError> for SpineError {
    fn from(e: us_tax_brackets::TaxError) -> Self {
        SpineError::TaxComputeError(e)
    }
}

impl fmt::Display for SpineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpineError::YearMismatch { input, rules } => {
                write!(f, "tax year mismatch: input={input}, rules={rules}")
            }
            SpineError::TaxComputeError(e) => write!(f, "tax computation error: {e}"),
        }
    }
}

impl std::error::Error for SpineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpineError::TaxComputeError(e) => Some(e),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Spine
// ---------------------------------------------------------------------------

/// Computes the core Form 1040 workflow and returns a [`Ledger`] of results.
///
/// Follows the standard 1040 flow:
/// Income → Adjustments → AGI → Deductions → Taxable Income → Regular Tax →
/// Additional Tax → Credits (nonrefundable / refundable) → Payments →
/// Refund or Amount Owed.
///
/// Returns [`SpineError::YearMismatch`] if `input.tax_year` differs from
/// `rules.year()`, or [`SpineError::TaxComputeError`] if the underlying
/// bracket lookup fails.
pub fn compute_spine(rules: &dyn TaxYearRules, input: &ReturnInput) -> Result<Ledger, SpineError> {
    if input.tax_year != rules.year() {
        return Err(SpineError::YearMismatch {
            input: input.tax_year,
            rules: rules.year(),
        });
    }

    // TODO: sum all income sources (interest, dividends, business, capital gains, etc.)
    let total_income = input.w2_wages;
    // TODO: Schedule 1 adjustments (educator expenses, HSA, IRA, student loan interest, etc.)
    let adjustments = Usd::ZERO;
    let agi = total_income - adjustments;

    // TODO: choose between standard and itemized deductions (Schedule A)
    let deductions = rules.standard_deduction(input.filing_status);
    let taxable_income = (agi - deductions).max(Usd::ZERO);

    // compute_tax expects whole dollars; convert via IRS rounding.
    let taxable_whole_dollars: i64 = taxable_income.irs_round().cents() / 100;
    let regular_tax_whole_dollars =
        us_tax_brackets::compute_tax(input.tax_year, input.filing_status, taxable_whole_dollars)?;
    let regular_tax = Usd::from_dollars(regular_tax_whole_dollars);

    // TODO: AMT, self-employment tax, additional Medicare, net investment income tax, etc.
    let additional_tax = Usd::ZERO;
    let total_tax_pre_credits = regular_tax + additional_tax;

    // TODO: child tax credit, education credits, foreign tax credit, etc.
    let nonrefundable_credits = Usd::ZERO;
    let tax_after_nonrefundable = (total_tax_pre_credits - nonrefundable_credits).max(Usd::ZERO);

    // TODO: EIC, additional child tax credit, American opportunity credit, etc.
    let refundable_credits = Usd::ZERO;
    let total_tax = tax_after_nonrefundable - refundable_credits;

    let withholding = input.fed_withholding;
    // TODO: estimated tax payments, amount applied from prior year, extension payments, etc.
    let estimated_payments = Usd::ZERO;
    let total_payments = withholding + estimated_payments;

    let net = total_payments - total_tax;
    let refund = net.max(Usd::ZERO);
    let owed = (Usd::ZERO - net).max(Usd::ZERO);

    let mut ledger = Ledger::new();
    ledger.insert(Key::TotalIncome, total_income);
    ledger.insert(Key::Adjustments, adjustments);
    ledger.insert(Key::AGI, agi);
    ledger.insert(Key::Deductions, deductions);
    ledger.insert(Key::TaxableIncome, taxable_income);
    ledger.insert(Key::RegularTax, regular_tax);
    ledger.insert(Key::AdditionalTax, additional_tax);
    ledger.insert(Key::TotalTaxPreCredits, total_tax_pre_credits);
    ledger.insert(Key::NonRefundableCredits, nonrefundable_credits);
    ledger.insert(Key::TaxAfterNonRefundableCredits, tax_after_nonrefundable);
    ledger.insert(Key::RefundableCredits, refundable_credits);
    ledger.insert(Key::TotalTax, total_tax);
    ledger.insert(Key::Withholding, withholding);
    ledger.insert(Key::EstimatedPayments, estimated_payments);
    ledger.insert(Key::TotalPayments, total_payments);
    ledger.insert(Key::Refund, refund);
    ledger.insert(Key::AmountOwed, owed);

    Ok(ledger)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::y2025::Rules2025;

    fn input(wages: i64, withholding: i64) -> ReturnInput {
        ReturnInput {
            tax_year: TaxYear::Y2025,
            filing_status: FilingStatus::Single,
            w2_wages: Usd::from_dollars(wages),
            fed_withholding: Usd::from_dollars(withholding),
        }
    }

    #[test]
    fn year_mismatch() {
        let inp = ReturnInput {
            tax_year: TaxYear::Y2024,
            filing_status: FilingStatus::Single,
            w2_wages: Usd::from_dollars(50_000),
            fed_withholding: Usd::ZERO,
        };
        let err = compute_spine(&Rules2025, &inp).unwrap_err();
        assert!(matches!(
            err,
            SpineError::YearMismatch {
                input: TaxYear::Y2024,
                rules: TaxYear::Y2025,
            }
        ));
    }

    #[test]
    fn wages_below_deduction_full_refund() {
        let ledger = compute_spine(&Rules2025, &input(10_000, 2_000)).unwrap();
        assert_eq!(ledger[&Key::TaxableIncome], Usd::ZERO);
        assert_eq!(ledger[&Key::RegularTax], Usd::ZERO);
        assert_eq!(ledger[&Key::TotalTax], Usd::ZERO);
        assert_eq!(ledger[&Key::Refund], Usd::from_dollars(2_000));
        assert_eq!(ledger[&Key::AmountOwed], Usd::ZERO);
    }

    #[test]
    fn wages_above_deduction_no_withholding_owes() {
        let ledger = compute_spine(&Rules2025, &input(50_000, 0)).unwrap();
        assert!(ledger[&Key::TaxableIncome] > Usd::ZERO);
        assert!(ledger[&Key::RegularTax] > Usd::ZERO);
        assert_eq!(ledger[&Key::Refund], Usd::ZERO);
        assert!(ledger[&Key::AmountOwed] > Usd::ZERO);
        assert_eq!(ledger[&Key::AmountOwed], ledger[&Key::TotalTax]);
    }

    #[test]
    fn withholding_exceeds_tax_refund() {
        let ledger = compute_spine(&Rules2025, &input(50_000, 10_000)).unwrap();
        let tax = ledger[&Key::TotalTax];
        assert!(tax > Usd::ZERO);
        assert!(Usd::from_dollars(10_000) > tax);
        assert!(ledger[&Key::Refund] > Usd::ZERO);
        assert_eq!(ledger[&Key::AmountOwed], Usd::ZERO);
        assert_eq!(ledger[&Key::Refund], Usd::from_dollars(10_000) - tax,);
    }

    #[test]
    fn ledger_has_all_keys() {
        let ledger = compute_spine(&Rules2025, &input(50_000, 5_000)).unwrap();
        let expected = [
            Key::TotalIncome,
            Key::Adjustments,
            Key::AGI,
            Key::Deductions,
            Key::TaxableIncome,
            Key::RegularTax,
            Key::AdditionalTax,
            Key::TotalTaxPreCredits,
            Key::NonRefundableCredits,
            Key::TaxAfterNonRefundableCredits,
            Key::RefundableCredits,
            Key::TotalTax,
            Key::Withholding,
            Key::EstimatedPayments,
            Key::TotalPayments,
            Key::Refund,
            Key::AmountOwed,
        ];
        for key in expected {
            assert!(ledger.contains_key(&key), "missing key: {key:?}");
        }
    }

    #[test]
    fn zero_wages_zero_withholding() {
        let ledger = compute_spine(&Rules2025, &input(0, 0)).unwrap();
        assert_eq!(ledger[&Key::TotalIncome], Usd::ZERO);
        assert_eq!(ledger[&Key::TaxableIncome], Usd::ZERO);
        assert_eq!(ledger[&Key::TotalTax], Usd::ZERO);
        assert_eq!(ledger[&Key::Refund], Usd::ZERO);
        assert_eq!(ledger[&Key::AmountOwed], Usd::ZERO);
    }
}
