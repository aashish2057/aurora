use crate::csv_utils::parse_csv;
use crate::transaction::{Account, ChaseAccount, Transaction};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct ChaseDepositAccountRow {
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
pub struct ChaseCreditCardAccountRow {
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

impl From<ChaseDepositAccountRow> for Transaction {
    fn from(row: ChaseDepositAccountRow) -> Self {
        Transaction {
            date: row._posting_date,
            account: Account::Chase(ChaseAccount::Deposit1199),
            description: row._description,
            category: String::new(),
            amount: row._amount,
        }
    }
}

impl From<ChaseCreditCardAccountRow> for Transaction {
    fn from(row: ChaseCreditCardAccountRow) -> Self {
        Transaction {
            date: row._transaction_date,
            account: Account::Chase(ChaseAccount::CreditCard9055),
            description: row._description,
            category: row._category.unwrap_or_default(),
            amount: row._amount,
        }
    }
}

pub fn parse_1199(path: &Path) -> Result<Vec<Transaction>, csv::Error> {
    parse_csv::<ChaseDepositAccountRow>(path).map(|rows| {
        rows.into_iter()
            .map(Transaction::from)
            .collect::<Vec<Transaction>>()
    })
}

pub fn parse_9055(path: &Path) -> Result<Vec<Transaction>, csv::Error> {
    parse_csv::<ChaseCreditCardAccountRow>(path).map(|rows| {
        rows.into_iter()
            .map(Transaction::from)
            .collect::<Vec<Transaction>>()
    })
}
