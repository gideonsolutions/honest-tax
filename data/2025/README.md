# 2025 Tax Year Data

Extracted from official IRS sources following the HonestTax research protocol.

## Sources

- **IRS Revenue Procedure 2024-40** - Original 2025 inflation adjustments (October 2024)
- **IRS Newsroom** - Tax Inflation Adjustments for Tax Year 2025
- **One Big Beautiful Bill Act** - Signed July 4, 2025; modified standard deduction and CTC amounts
- **IRS Schedule 8812 Instructions (2025)** - Child tax credit details
- **IRS Topic 551** - Standard deduction reference

## Files

| File | Description |
|------|-------------|
| `tax_brackets.json` | Federal income tax rates and bracket thresholds for all filing statuses |
| `standard_deduction.json` | Base and additional standard deduction amounts |
| `credits.json` | Child Tax Credit, Credit for Other Dependents, EITC |
| `other_parameters.json` | AMT, estate/gift, FSA, transportation fringe benefits |

## Key Changes for 2025

1. **Standard Deduction** increased by One Big Beautiful Bill Act:
   - Single: $15,750 (was $15,000 in Rev. Proc. 2024-40)
   - MFJ: $31,500 (was $30,000)
   - HoH: $23,625 (was $22,500)

2. **Child Tax Credit** increased to $2,200 per child (was $2,000)

3. **New Senior Deduction** ($6,000 per person age 65+) for 2025-2028

4. **TCJA provisions made permanent** by One Big Beautiful Bill Act

## UNKNOWN Values

Per the research protocol, values marked as `"UNKNOWN"` could not be confirmed from explicit IRS source text:
- EITC maximum amounts for 0, 1, and 2 qualifying children (only 3+ children amount confirmed)

## Extraction Date

2026-01-25
