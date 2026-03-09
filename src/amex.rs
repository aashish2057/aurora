use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AmexGoldFormat {
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
