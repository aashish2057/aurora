use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct VentureXFormat {
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
struct Checking360Format {
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
struct AashishRainyDayFormat {
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
struct ParentsRainyDayFormat {
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
struct AmexGoldFormat {
    #[serde(rename = "Date")]
    _date: String,
    #[serde(rename = "Description")]
    _description: String,
    #[serde(rename = "Amount")]
    _amount: f32,
    #[serde(rename = "Extended Details")]
    _extended_details: String,
    #[serde(rename = "Appears On Your Statement As")]
    _appears_on_your_statement_as: String,
    #[serde(rename = "Address")]
    _address: String,
    #[serde(rename = "City/State")]
    _city_state: String,
    #[serde(rename = "Zip Code")]
    _zip_code: String,
    #[serde(rename = "Country")]
    _country: String,
    #[serde(rename = "Reference")]
    _reference: String,
    #[serde(rename = "Category")]
    _category: String,
}

#[derive(Debug, Deserialize)]
struct Chase1199Format {
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
struct Chase9055Format {
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

fn main() {
    let base_path = "/Users/aashishsharma/Dev/Jan 2026/";

    let files = [
        "2026-02-28_transaction_download_venture_x.csv",
        "2026-02-28_360Checking...5180.csv",
        "2026-02-28_AashishRainyDay...5723.csv",
        "2026-02-28_ParentsRainyDay...1260.csv",
        "AMEX GOLD - 01:2026.csv",
        "Chase1199_Activity_20260228.CSV",
        "Chase9055_Activity20260101_20260131_20260228.CSV",
    ];

    for file in files {
        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .from_path(format!("{base_path}{file}"))
            .expect("file not found");

        println!("FILE NAME: {file}");

        match file {
            "2026-02-28_transaction_download_venture_x.csv" => {
                for result in rdr.deserialize() {
                    let record: VentureXFormat = result.expect("a CSV record");
                    println!("{:?}", record);
                }
            }
            "2026-02-28_360Checking...5180.csv" => {
                for result in rdr.deserialize() {
                    let record: Checking360Format = result.expect("a CSV record");
                    println!("{:?}", record);
                }
            }
            "2026-02-28_AashishRainyDay...5723.csv" => {
                for result in rdr.deserialize() {
                    let record: AashishRainyDayFormat = result.expect("a CSV record");
                    println!("{:?}", record);
                }
            }
            "2026-02-28_ParentsRainyDay...1260.csv" => {
                for result in rdr.deserialize() {
                    let record: ParentsRainyDayFormat = result.expect("a CSV record");
                    println!("{:?}", record);
                }
            }
            "AMEX GOLD - 01:2026.csv" => {
                for result in rdr.deserialize() {
                    let record: AmexGoldFormat = result.expect("a CSV record");
                    println!("{:?}", record);
                }
            }
            "Chase1199_Activity_20260228.CSV" => {
                for result in rdr.deserialize() {
                    let record: Chase1199Format = result.expect("a CSV record");
                    println!("{:?}", record);
                }
            }
            "Chase9055_Activity20260101_20260131_20260228.CSV" => {
                for result in rdr.deserialize() {
                    let record: Chase9055Format = result.expect("a CSV record");
                    println!("{:?}", record);
                }
            }
            _ => panic!("Unsupported file: {file}"),
        }

        println!();
        println!();
    }
}
