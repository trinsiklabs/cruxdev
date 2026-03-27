# Invoice Template

**Template Version:** 1.0
**Last Updated:** 2026-03-24

---

## INVOICE

| | |
|---|---|
| **Invoice Number:** | {{INVOICE_PREFIX}}-{{INVOICE_NUMBER}} |
| **Invoice Date:** | {{INVOICE_DATE}} |
| **Due Date:** | {{DUE_DATE}} |
| **Payment Terms:** | {{PAYMENT_TERMS: Net 30 / Net 15 / Due on Receipt}} |

---

### FROM:

**{{COMPANY_LEGAL_NAME}}**
{{COMPANY_ADDRESS_LINE_1}}
{{COMPANY_ADDRESS_LINE_2}}
{{COMPANY_CITY}}, {{COMPANY_STATE}} {{COMPANY_ZIP}}
{{COMPANY_COUNTRY}}

Phone: {{COMPANY_PHONE}}
Email: {{BILLING_EMAIL}}
Tax ID: {{TAX_ID}}

---

### BILL TO:

**{{CLIENT_LEGAL_NAME}}**
{{CLIENT_ADDRESS_LINE_1}}
{{CLIENT_ADDRESS_LINE_2}}
{{CLIENT_CITY}}, {{CLIENT_STATE}} {{CLIENT_ZIP}}
{{CLIENT_COUNTRY}}

Attn: {{CLIENT_CONTACT}}
Email: {{CLIENT_EMAIL}}
PO Number: {{PO_NUMBER}} *(if applicable)*

---

### INVOICE DETAILS

| # | Description | Quantity | Unit | Rate | Amount |
|---|-------------|----------|------|------|--------|
| 1 | {{LINE_ITEM_1}} | {{QTY_1}} | {{UNIT_1: hours / units / months}} | ${{RATE_1}} | ${{AMOUNT_1}} |
| 2 | {{LINE_ITEM_2}} | {{QTY_2}} | {{UNIT_2}} | ${{RATE_2}} | ${{AMOUNT_2}} |
| 3 | {{LINE_ITEM_3}} | {{QTY_3}} | {{UNIT_3}} | ${{RATE_3}} | ${{AMOUNT_3}} |
| 4 | {{LINE_ITEM_4}} | {{QTY_4}} | {{UNIT_4}} | ${{RATE_4}} | ${{AMOUNT_4}} |

---

| | |
|---|---|
| **Subtotal:** | ${{SUBTOTAL}} |
| **Discount ({{DISCOUNT_DESCRIPTION}}):** | -${{DISCOUNT_AMOUNT}} |
| **Tax ({{TAX_RATE}}%):** | ${{TAX_AMOUNT}} |
| **TOTAL DUE:** | **${{TOTAL_DUE}}** |

**Currency:** {{CURRENCY}}

---

### PAYMENT INSTRUCTIONS

**Bank Transfer (ACH/Wire):**

| Field | Value |
|-------|-------|
| Bank Name | {{BANK_NAME}} |
| Account Name | {{ACCOUNT_NAME}} |
| Account Number | {{ACCOUNT_NUMBER}} |
| Routing Number (ACH) | {{ROUTING_NUMBER}} |
| SWIFT/BIC (international) | {{SWIFT_CODE}} |

**Other Payment Methods:**
- [ ] Check payable to: {{COMPANY_LEGAL_NAME}}
- [ ] Credit card: Contact {{BILLING_EMAIL}}
- [ ] Online payment: {{PAYMENT_URL}}

**Please include invoice number {{INVOICE_PREFIX}}-{{INVOICE_NUMBER}} with your payment.**

---

### TERMS AND CONDITIONS

1. Payment is due by {{DUE_DATE}}. Late payments are subject to a {{LATE_FEE}}% monthly finance charge.
2. This invoice is subject to the terms of {{REFERENCE_AGREEMENT: our Service Agreement dated X / our standard terms}}.
3. Questions about this invoice should be directed to {{BILLING_EMAIL}} within {{DISPUTE_PERIOD}} of the invoice date.

---

### NOTES

{{INVOICE_NOTES}}

---

*Thank you for your business.*

---

## INVOICE TRACKING LOG

*(Internal use — do not send to client)*

| Invoice # | Client | Amount | Sent Date | Due Date | Status | Paid Date | Payment Ref |
|-----------|--------|--------|-----------|----------|--------|-----------|-------------|
| {{INV_1}} | {{CLIENT_1}} | ${{AMT_1}} | {{SENT_1}} | {{DUE_1}} | {{STATUS_1: Sent / Paid / Overdue / Disputed}} | {{PAID_1}} | {{REF_1}} |

**Aging Summary:**

| Bucket | Count | Amount |
|--------|-------|--------|
| Current (0-30) | | |
| 31-60 days | | |
| 61-90 days | | |
| 90+ days | | |
| **Total Outstanding** | | |
