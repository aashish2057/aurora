use crate::csv_utils::parse_csv;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Chase1199Format {
    #[serde(rename = "Details")]
    _details: String,
    #[serde(rename = "Posting Date")]
    _posting_date: String,
    #[serde(rename = "Description")]
    _description: String,
    #[serde(rename = "Amount")]
    _amount: f32,
    #[serde(rename = "Type")]
    _type: String,
    #[serde(rename = "Balance")]
    _balance: f32,
    #[serde(rename = "Check or Slip #")]
    _check_or_slip_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Chase9055Format {
    #[serde(rename = "Transaction Date")]
    _transaction_date: String,
    #[serde(rename = "Post Date")]
    _post_date: String,
    #[serde(rename = "Description")]
    _description: String,
    #[serde(rename = "Category")]
    _category: Option<String>,
    #[serde(rename = "Type")]
    _type: String,
    #[serde(rename = "Amount")]
    _amount: f32,
    #[serde(rename = "Memo")]
    _memo: Option<String>,
}

pub(crate) fn parse_1199(path: &Path) -> Result<Vec<Chase1199Format>, csv::Error> {
    parse_csv(path)
}

pub(crate) fn parse_9055(path: &Path) -> Result<Vec<Chase9055Format>, csv::Error> {
    parse_csv(path)
}
