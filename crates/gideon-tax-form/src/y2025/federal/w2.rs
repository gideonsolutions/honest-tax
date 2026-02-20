use gideon_tax_core::Usd;

/// IRS Form W-2: Wage and Tax Statement (2025).
#[derive(Debug, Clone, Default)]
pub struct IrsW2 {
    // -- Identifiers / header --
    pub employee_ssn: Option<String>,
    pub employee_nm: Option<String>,
    pub employer_ein: Option<String>,
    pub employer_name: Option<EmployerName>,
    pub control_num: Option<String>,
    pub employer_name_control_txt: Option<String>,

    // -- Addresses --
    pub employee_us_address: Option<UsAddress>,
    pub employee_foreign_address: Option<String>,
    pub employer_us_address: Option<UsAddress>,
    pub employer_foreign_address: Option<String>,

    // -- Dollar amounts (boxes 1–11) --
    /// Box 1: Wages, tips, other compensation.
    pub wages_amt: Option<Usd>,
    /// Box 2: Federal income tax withheld.
    pub withholding_amt: Option<Usd>,
    /// Box 3: Social security wages.
    pub social_security_wages_amt: Option<Usd>,
    /// Box 4: Social security tax withheld.
    pub social_security_tax_amt: Option<Usd>,
    /// Box 5: Medicare wages and tips.
    pub medicare_wages_and_tips_amt: Option<Usd>,
    /// Box 6: Medicare tax withheld.
    pub medicare_tax_withheld_amt: Option<Usd>,
    /// Box 7: Social security tips.
    pub social_security_tips_amt: Option<Usd>,
    /// Box 8: Allocated tips.
    pub allocated_tips_amt: Option<Usd>,
    /// Box 10: Dependent care benefits.
    pub dependent_care_benefits_amt: Option<Usd>,
    /// Box 11: Nonqualified plans.
    pub nonqualified_plans_amt: Option<Usd>,

    // -- Checkboxes (box 13) --
    pub statutory_employee_ind: Option<bool>,
    pub retirement_plan_ind: Option<bool>,
    pub third_party_sick_pay_ind: Option<bool>,

    // -- Indicators --
    pub corrected_w2_ind: Option<bool>,
    pub agent_for_employer_ind: Option<bool>,

    // -- Coded / grouped sections --
    /// Box 12: Coded entries (e.g. 401k, HSA contributions).
    pub box_12: Option<Vec<Box12Entry>>,
    /// Box 14: Other deductions and benefits.
    pub other_deductions_benefits_grp: Option<Vec<OtherDeductionBenefit>>,
    /// Boxes 15–20: State and local tax information.
    pub w2_state_local_tax_grp: Option<Vec<W2StateTaxGrp>>,
    pub standard_or_non_standard_cd: Option<StandardOrNonStandardCd>,
    pub employers_use_grp: Option<EmployersUseGrp>,

    // -- Security (e-file metadata, not taxpayer-facing) --
    pub w2_security_information: Option<W2SecurityInformation>,
}

// ---------------------------------------------------------------------------
// Sub-structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct UsAddress {
    pub address_line_1_txt: Option<String>,
    pub address_line_2_txt: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EmployerName {
    pub business_name_line_1_txt: Option<String>,
    pub business_name_line_2_txt: Option<String>,
}

/// Box 12 code + amount pair.
#[derive(Debug, Clone)]
pub struct Box12Entry {
    pub code: Box12Code,
    pub amount: Usd,
}

/// Box 14 description + amount pair.
#[derive(Debug, Clone)]
pub struct OtherDeductionBenefit {
    pub description: Option<String>,
    pub amount: Option<Usd>,
}

/// Boxes 15–17: State-level withholding group.
#[derive(Debug, Clone, Default)]
pub struct W2StateTaxGrp {
    pub state_abbrev: Option<String>,
    pub employer_state_id: Option<String>,
    pub state_wages_amt: Option<Usd>,
    pub state_income_tax_amt: Option<Usd>,
    /// Boxes 18–20: Local-level withholding within this state.
    pub w2_local_tax_grp: Option<Vec<W2LocalTaxGrp>>,
}

/// Boxes 18–20: Local-level withholding group.
#[derive(Debug, Clone, Default)]
pub struct W2LocalTaxGrp {
    pub local_wages_amt: Option<Usd>,
    pub local_income_tax_amt: Option<Usd>,
    pub locality_nm: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EmployersUseGrp {}

#[derive(Debug, Clone, Default)]
pub struct W2SecurityInformation {
    pub w2_download_cd: Option<String>,
    pub w2_download_failed_attempt_cnt: Option<i32>,
    pub w2_download_result_cd: Option<String>,
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardOrNonStandardCd {
    Standard,
    NonStandard,
}

/// W-2 Box 12 codes (A through HH).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Box12Code {
    /// Uncollected social security or RRTA tax on tips.
    A,
    /// Uncollected Medicare tax on tips.
    B,
    /// Taxable cost of group-term life insurance over $50,000.
    C,
    /// Elective deferrals under a section 401(k) plan.
    D,
    /// Elective deferrals under a section 403(b) plan.
    E,
    /// Elective deferrals under a section 408(k)(6) SIMPLE plan.
    F,
    /// Elective deferrals and employer contributions to a section 457(b) plan.
    G,
    /// Elective deferrals under a section 501(c)(18)(D) plan.
    H,
    /// Sick pay not includible in income (third-party payer only).
    J,
    /// 20% excise tax on excess golden parachute payments.
    K,
    /// Substantiated employee business expense reimbursements.
    L,
    /// Uncollected social security or RRTA tax on group-term life insurance.
    M,
    /// Uncollected Medicare tax on group-term life insurance.
    N,
    /// Exempt TRICARE supplemental insurance premiums.
    P,
    /// Nontaxable combat pay.
    Q,
    /// Employer contributions to Archer MSA.
    R,
    /// Employee salary reduction contributions under section 408(p) SIMPLE.
    S,
    /// Adoption benefits.
    T,
    /// Income from exercise of nonstatutory stock option(s).
    V,
    /// Employer contributions to HSA.
    W,
    /// Deferrals under a section 409A nonqualified deferred compensation plan.
    Y,
    /// Income under a section 409A nonqualified plan that fails section 409A.
    Z,
    /// Designated Roth contributions under a section 401(k) plan.
    AA,
    /// Designated Roth contributions under a section 403(b) plan.
    BB,
    /// Cost of employer-sponsored health coverage (informational).
    DD,
    /// Designated Roth contributions under a governmental section 457(b) plan.
    EE,
    /// Qualified small employer health reimbursement arrangement.
    FF,
    /// Income from qualified equity grants under section 83(i).
    GG,
    /// Aggregate deferrals under section 83(i) elections as of close of calendar year.
    HH,
}
