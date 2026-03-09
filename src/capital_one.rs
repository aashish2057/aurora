use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VentureXFormat {
    #[serde(rename = "Transaction Date")]
    _transaction_date: String,
    #[serde(rename = "Posted Date")]
    _posted_date: String,
    #[serde(rename = "Card No.")]
    _card_number: String,
    #[serde(rename = "Description")]
    _description: String,
    #[serde(rename = "Category")]
    _category: String,
    #[serde(rename = "Debit")]
    _debit: Option<f32>,
    #[serde(rename = "Credit")]
    _credit: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct Checking360Format {
    #[serde(rename = "Account Number")]
    _account_number: String,
    #[serde(rename = "Transaction Description")]
    _transaction_description: String,
    #[serde(rename = "Transaction Date")]
    _transaction_date: String,
    #[serde(rename = "Transaction Type")]
    _transaction_type: String,
    #[serde(rename = "Transaction Amount")]
    _transaction_amount: f32,
    #[serde(rename = "Balance")]
    _balance: f32,
}

#[derive(Debug, Deserialize)]
pub struct AashishRainyDayFormat {
    #[serde(rename = "Account Number")]
    _account_number: String,
    #[serde(rename = "Transaction Description")]
    _transaction_description: String,
    #[serde(rename = "Transaction Date")]
    _transaction_date: String,
    #[serde(rename = "Transaction Type")]
    _transaction_type: String,
    #[serde(rename = "Transaction Amount")]
    _transaction_amount: f32,
    #[serde(rename = "Balance")]
    _balance: f32,
}

#[derive(Debug, Deserialize)]
pub struct ParentsRainyDayFormat {
    #[serde(rename = "Account Number")]
    _account_number: String,
    #[serde(rename = "Transaction Description")]
    _transaction_description: String,
    #[serde(rename = "Transaction Date")]
    _transaction_date: String,
    #[serde(rename = "Transaction Type")]
    _transaction_type: String,
    #[serde(rename = "Transaction Amount")]
    _transaction_amount: f32,
    #[serde(rename = "Balance")]
    _balance: f32,
}
