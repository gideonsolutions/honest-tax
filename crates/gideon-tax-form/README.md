# gideon-tax-form

US tax form schemas for individual income tax filing.

## Schema Organization

Schemas live under `schemas/<year>/federal/` and are split into two categories:

### `source/` — Information Documents

Forms received from third parties (employers, banks, brokers, government agencies).
The filer does **not** fill these out. They provide the raw data needed to prepare a
tax return. The IRS calls these "information returns" because third parties file them
with the IRS independently.

### `return/` — Tax Return Forms

Forms the filer fills out and submits to the IRS. Together these constitute the
individual's **tax return**.

## 2025 Federal Forms

### Source Forms

| File | Form | Title |
|------|------|-------|
| `w-2.toml` | W-2 | Wage and Tax Statement |
| `w-2g.toml` | W-2G | Certain Gambling Winnings |
| `1098.toml` | 1098 | Mortgage Interest Statement |
| `1098-e.toml` | 1098-E | Student Loan Interest Statement |
| `1098-t.toml` | 1098-T | Tuition Statement |
| `1099-int.toml` | 1099-INT | Interest Income |
| `1099-div.toml` | 1099-DIV | Dividends and Distributions |
| `1099-oid.toml` | 1099-OID | Original Issue Discount |
| `1099-b.toml` | 1099-B | Proceeds From Broker and Barter Exchange Transactions |
| `1099-r.toml` | 1099-R | Distributions From Pensions, Annuities, Retirement or Profit-Sharing Plans, IRAs, Insurance Contracts, etc. |
| `1099-g.toml` | 1099-G | Certain Government Payments |
| `1099-misc.toml` | 1099-MISC | Miscellaneous Information |
| `1099-nec.toml` | 1099-NEC | Nonemployee Compensation |
| `1099-s.toml` | 1099-S | Proceeds From Real Estate Transactions |
| `1099-sa.toml` | 1099-SA | Distributions From an HSA, Archer MSA, or Medicare Advantage MSA |
| `1099-q.toml` | 1099-Q | Payments From Qualified Education Programs |
| `1099-c.toml` | 1099-C | Cancellation of Debt |
| `1099-k.toml` | 1099-K | Payment Card and Third Party Network Transactions |
| `1099-ltc.toml` | 1099-LTC | Long-Term Care and Accelerated Death Benefits |
| `1099-patr.toml` | 1099-PATR | Taxable Distributions Received From Cooperatives |
| `ssa-1099.toml` | SSA-1099 | Social Security Benefit Statement |
| `rrb-1099.toml` | RRB-1099 | Payments by the Railroad Retirement Board |
| `schedule-k-1-1065.toml` | Schedule K-1 (Form 1065) | Partner's Share of Income, Deductions, Credits, etc. |
| `schedule-k-1-1120-s.toml` | Schedule K-1 (Form 1120-S) | Shareholder's Share of Income, Deductions, Credits, etc. |
| `schedule-k-1-1041.toml` | Schedule K-1 (Form 1041) | Beneficiary's Share of Income, Deductions, Credits, etc. |
| `5498.toml` | 5498 | IRA Contribution Information |
| `5498-sa.toml` | 5498-SA | HSA, Archer MSA, or Medicare Advantage MSA Information |

### Return Forms

| File | Form | Title |
|------|------|-------|
| `1040.toml` | 1040 | U.S. Individual Income Tax Return |
| `schedule-1.toml` | Schedule 1 (Form 1040) | Additional Income and Adjustments to Income |
| `schedule-2.toml` | Schedule 2 (Form 1040) | Additional Taxes |
| `schedule-3.toml` | Schedule 3 (Form 1040) | Additional Credits and Payments |
| `schedule-a.toml` | Schedule A (Form 1040) | Itemized Deductions |
| `schedule-b.toml` | Schedule B (Form 1040) | Interest and Ordinary Dividends |
| `schedule-c.toml` | Schedule C (Form 1040) | Profit or Loss From Business (Sole Proprietorship) |
| `schedule-d.toml` | Schedule D (Form 1040) | Capital Gains and Losses |
| `schedule-e.toml` | Schedule E (Form 1040) | Supplemental Income and Loss |
| `schedule-f.toml` | Schedule F (Form 1040) | Profit or Loss From Farming |
| `schedule-h.toml` | Schedule H (Form 1040) | Household Employment Taxes |
| `schedule-j.toml` | Schedule J (Form 1040) | Income Averaging for Farmers and Fishermen |
| `schedule-r.toml` | Schedule R (Form 1040) | Credit for the Elderly or the Disabled |
| `schedule-eic.toml` | Schedule EIC (Form 1040) | Earned Income Credit |
| `schedule-se.toml` | Schedule SE (Form 1040) | Self-Employment Tax |
| `8949.toml` | Form 8949 | Sales and Other Dispositions of Capital Assets |
| `8959.toml` | Form 8959 | Additional Medicare Tax |
| `8960.toml` | Form 8960 | Net Investment Income Tax |
| `6251.toml` | Form 6251 | Alternative Minimum Tax — Individuals |
| `8812.toml` | Form 8812 | Credits for Qualifying Children and Other Dependents |
| `2441.toml` | Form 2441 | Child and Dependent Care Expenses |
| `8863.toml` | Form 8863 | Education Credits |
| `8880.toml` | Form 8880 | Credit for Qualified Retirement Savings Contributions |
| `8962.toml` | Form 8962 | Premium Tax Credit |
| `8995.toml` | Form 8995 | Qualified Business Income Deduction Simplified Computation |
| `8995-a.toml` | Form 8995-A | Qualified Business Income Deduction |
| `8889.toml` | Form 8889 | Health Savings Accounts |
| `8606.toml` | Form 8606 | Nondeductible IRAs |
| `1116.toml` | Form 1116 | Foreign Tax Credit |
| `2555.toml` | Form 2555 | Foreign Earned Income |
| `4562.toml` | Form 4562 | Depreciation and Amortization |
| `4797.toml` | Form 4797 | Sales of Business Property |
| `8829.toml` | Form 8829 | Expenses for Business Use of Your Home |
| `5695.toml` | Form 5695 | Residential Energy Credits |
| `8936.toml` | Form 8936 | Qualified Plug-in Electric Drive Motor Vehicle Credit |
| `4868.toml` | Form 4868 | Application for Automatic Extension of Time to File |
| `9465.toml` | Form 9465 | Installment Agreement Request |
