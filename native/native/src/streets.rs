use std::path::PathBuf;

use csv::Reader;

#[derive(Debug, serde::Deserialize)]
pub struct Street {
    pub street_id: String,
    pub street_prefix: String,
    pub street_name: String,
}

pub fn read_streets_csv(mut path: PathBuf) -> Vec<Street> {
    path.push("ulice_wroclaw.csv");
    eprintln!("Reading streets from: {:?}", &path);

    let reader = Reader::from_path(path).unwrap();
    let data = reader
        .into_deserialize::<Street>()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    eprintln!("Streets loaded");

    return data;
}
