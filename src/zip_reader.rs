use std::fs::File;
use zip::ZipArchive;

pub fn unzip(path: String) {
    let file = File::open(path).expect("Could not open file!");
    let archive = ZipArchive::new(file).unwrap();

    // Read in zip file and recursively deserialize using serde_json
    todo!()
}