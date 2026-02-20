use crate::Usd;

// ---------------------------------------------------------------------------
// Part I — Additional Income (lines 1–10)
// ---------------------------------------------------------------------------

/// Additional income sources from Schedule 1, Part I.
///
/// Each field corresponds to a line on the form. The `total()` method
/// produces line 10, which feeds into Form 1040 total income.
#[derive(Debug, Clone, Default)]
pub struct AdditionalIncome {
    // Lines 1–7
    pub taxable_refunds: Usd,
    pub alimony_received: Usd,
    pub business_income: Usd,
    pub other_gains: Usd,
    pub rental_real_estate: Usd,
    pub farm_income: Usd,
    pub unemployment_compensation: Usd,

    // Lines 8a–8z (other income)
    pub nol_deduction: Usd,
    pub gambling_income: Usd,
    pub cancellation_of_debt: Usd,
    pub foreign_earned_income_exclusion: Usd,
    pub income_form_8853: Usd,
    pub income_form_8889: Usd,
    pub alaska_permanent_fund: Usd,
    pub jury_duty_pay: Usd,
    pub prizes_and_awards: Usd,
    pub activity_not_for_profit: Usd,
    pub stock_options: Usd,
    pub rental_personal_property: Usd,
    pub olympic_medals: Usd,
    pub section_951a_inclusion: Usd,
    pub section_951a_a_inclusion: Usd,
    pub excess_business_loss_adj: Usd,
    pub able_distributions: Usd,
    pub scholarship_grants: Usd,
    pub medicaid_waiver: Usd,
    pub nonqualified_deferred_comp: Usd,
    pub wages_while_incarcerated: Usd,
    pub digital_assets: Usd,
    pub other_income: Usd,
}

impl AdditionalIncome {
    /// Sum of lines 8a–8z → line 9.
    pub fn total_other_income(&self) -> Usd {
        self.nol_deduction
            + self.gambling_income
            + self.cancellation_of_debt
            + self.foreign_earned_income_exclusion
            + self.income_form_8853
            + self.income_form_8889
            + self.alaska_permanent_fund
            + self.jury_duty_pay
            + self.prizes_and_awards
            + self.activity_not_for_profit
            + self.stock_options
            + self.rental_personal_property
            + self.olympic_medals
            + self.section_951a_inclusion
            + self.section_951a_a_inclusion
            + self.excess_business_loss_adj
            + self.able_distributions
            + self.scholarship_grants
            + self.medicaid_waiver
            + self.nonqualified_deferred_comp
            + self.wages_while_incarcerated
            + self.digital_assets
            + self.other_income
    }

    /// Sum of lines 1–7 + line 9 → line 10.
    pub fn total(&self) -> Usd {
        self.taxable_refunds
            + self.alimony_received
            + self.business_income
            + self.other_gains
            + self.rental_real_estate
            + self.farm_income
            + self.unemployment_compensation
            + self.total_other_income()
    }
}

// ---------------------------------------------------------------------------
// Part II — Adjustments to Income (lines 11–26)
// ---------------------------------------------------------------------------

/// Adjustments to income from Schedule 1, Part II.
///
/// Each field corresponds to a line on the form. The `total()` method
/// produces line 26, which is subtracted from total income to arrive at AGI.
#[derive(Debug, Clone, Default)]
pub struct Adjustments {
    // Lines 11–23
    pub educator_expenses: Usd,
    pub business_expenses_reservists: Usd,
    pub hsa_deduction: Usd,
    pub moving_expenses: Usd,
    pub se_tax_deduction: Usd,
    pub se_retirement_plans: Usd,
    pub se_health_insurance: Usd,
    pub early_withdrawal_penalty: Usd,
    pub alimony_paid: Usd,
    pub ira_deduction: Usd,
    pub student_loan_interest: Usd,
    pub archer_msa_deduction: Usd,

    // Lines 24a–24z (other adjustments)
    pub jury_duty_pay: Usd,
    pub rental_personal_property: Usd,
    pub olympic_medals: Usd,
    pub reforestation: Usd,
    pub supplemental_unemployment: Usd,
    pub contributions_501c18d: Usd,
    pub chaplain_contributions: Usd,
    pub attorney_fees_discrimination: Usd,
    pub attorney_fees_whistleblower: Usd,
    pub housing_deduction_2555: Usd,
    pub excess_deductions_67e: Usd,
    pub other_adjustments: Usd,
}

impl Adjustments {
    /// Sum of lines 24a–24z → line 25.
    pub fn total_other_adjustments(&self) -> Usd {
        self.jury_duty_pay
            + self.rental_personal_property
            + self.olympic_medals
            + self.reforestation
            + self.supplemental_unemployment
            + self.contributions_501c18d
            + self.chaplain_contributions
            + self.attorney_fees_discrimination
            + self.attorney_fees_whistleblower
            + self.housing_deduction_2555
            + self.excess_deductions_67e
            + self.other_adjustments
    }

    /// Sum of lines 11–23 + line 25 → line 26.
    pub fn total(&self) -> Usd {
        self.educator_expenses
            + self.business_expenses_reservists
            + self.hsa_deduction
            + self.moving_expenses
            + self.se_tax_deduction
            + self.se_retirement_plans
            + self.se_health_insurance
            + self.early_withdrawal_penalty
            + self.alimony_paid
            + self.ira_deduction
            + self.student_loan_interest
            + self.archer_msa_deduction
            + self.total_other_adjustments()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_additional_income_is_zero() {
        let ai = AdditionalIncome::default();
        assert_eq!(ai.total_other_income(), Usd::ZERO);
        assert_eq!(ai.total(), Usd::ZERO);
    }

    #[test]
    fn default_adjustments_is_zero() {
        let adj = Adjustments::default();
        assert_eq!(adj.total_other_adjustments(), Usd::ZERO);
        assert_eq!(adj.total(), Usd::ZERO);
    }

    #[test]
    fn total_other_income_sums_8a_through_8z() {
        let ai = AdditionalIncome {
            gambling_income: Usd::from_dollars(500),
            prizes_and_awards: Usd::from_dollars(200),
            digital_assets: Usd::from_dollars(300),
            ..Default::default()
        };
        assert_eq!(ai.total_other_income(), Usd::from_dollars(1_000));
    }

    #[test]
    fn total_income_sums_lines_1_through_7_plus_other() {
        let ai = AdditionalIncome {
            business_income: Usd::from_dollars(10_000),
            unemployment_compensation: Usd::from_dollars(5_000),
            gambling_income: Usd::from_dollars(1_000),
            ..Default::default()
        };
        assert_eq!(ai.total_other_income(), Usd::from_dollars(1_000));
        assert_eq!(ai.total(), Usd::from_dollars(16_000));
    }

    #[test]
    fn total_other_adjustments_sums_24a_through_24z() {
        let adj = Adjustments {
            reforestation: Usd::from_dollars(100),
            attorney_fees_discrimination: Usd::from_dollars(400),
            ..Default::default()
        };
        assert_eq!(adj.total_other_adjustments(), Usd::from_dollars(500));
    }

    #[test]
    fn adjustments_total_sums_lines_11_through_23_plus_other() {
        let adj = Adjustments {
            educator_expenses: Usd::from_dollars(300),
            hsa_deduction: Usd::from_dollars(4_150),
            student_loan_interest: Usd::from_dollars(2_500),
            reforestation: Usd::from_dollars(50),
            ..Default::default()
        };
        assert_eq!(adj.total_other_adjustments(), Usd::from_dollars(50));
        assert_eq!(adj.total(), Usd::from_dollars(7_000));
    }
}
