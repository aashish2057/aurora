use crate::csv_utils::parse_csv;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct CapitalOneCreditCardAccountRow {
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
pub struct CapitalOneDepositAccountRow {
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

pub(crate) fn parse_venture_x(
    path: &Path,
) -> Result<Vec<CapitalOneCreditCardAccountRow>, csv::Error> {
    parse_csv(path)
}

pub(crate) fn parse_360_checking(
    path: &Path,
) -> Result<Vec<CapitalOneDepositAccountRow>, csv::Error> {
    parse_csv(path)
}

pub(crate) fn parse_aashish_rainy_day(
    path: &Path,
) -> Result<Vec<CapitalOneDepositAccountRow>, csv::Error> {
    parse_csv(path)
}

pub(crate) fn parse_parents_rainy_day(
    path: &Path,
) -> Result<Vec<CapitalOneDepositAccountRow>, csv::Error> {
    parse_csv(path)
}
