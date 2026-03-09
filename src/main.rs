mod amex;
mod capital_one;
mod chase;

use crate::{
    amex::AmexGoldFormat,
    capital_one::{
        AashishRainyDayFormat, Checking360Format, ParentsRainyDayFormat, VentureXFormat,
    },
    chase::{Chase1199Format, Chase9055Format},
};

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
