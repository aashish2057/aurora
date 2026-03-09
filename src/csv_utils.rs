use serde::de::DeserializeOwned;
use std::path::Path;

pub(crate) fn parse_csv<T>(path: &Path) -> Result<Vec<T>, csv::Error>
where
    T: DeserializeOwned,
{
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_path(path)?;

    rdr.deserialize::<T>().collect()
}
