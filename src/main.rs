mod amex;
mod capital_one;
mod chase;
mod csv_utils;

use std::{fmt::Debug, path::Path};

fn print_records<T: Debug>(records: Vec<T>) {
    for record in records {
        println!("{:?}", record);
    }
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
        let path = Path::new(base_path).join(file);

        println!("FILE NAME: {file}");

        match file {
            "2026-02-28_transaction_download_venture_x.csv" => {
                print_records(capital_one::parse_venture_x(path.as_path()).expect("a CSV record"));
            }
            "2026-02-28_360Checking...5180.csv" => {
                print_records(
                    capital_one::parse_360_checking(path.as_path()).expect("a CSV record"),
                );
            }
            "2026-02-28_AashishRainyDay...5723.csv" => {
                print_records(
                    capital_one::parse_aashish_rainy_day(path.as_path()).expect("a CSV record"),
                );
            }
            "2026-02-28_ParentsRainyDay...1260.csv" => {
                print_records(
                    capital_one::parse_parents_rainy_day(path.as_path()).expect("a CSV record"),
                );
            }
            "AMEX GOLD - 01:2026.csv" => {
                print_records(amex::parse_gold(path.as_path()).expect("a CSV record"));
            }
            "Chase1199_Activity_20260228.CSV" => {
                print_records(chase::parse_1199(path.as_path()).expect("a CSV record"));
            }
            "Chase9055_Activity20260101_20260131_20260228.CSV" => {
                print_records(chase::parse_9055(path.as_path()).expect("a CSV record"));
            }
            _ => panic!("Unsupported file: {file}"),
        }

        println!();
        println!();
    }
}
