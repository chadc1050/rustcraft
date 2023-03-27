use std::{fs, io};
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;
use zip::ZipArchive;

pub fn unzip_assets(path: &str) {

    let begin = SystemTime::now();

    let file_name = Path::new(path);
    let file = File::open(file_name).expect("Could not open file!");
    let mut archive = ZipArchive::new(file).unwrap();

    let root = Path::new("assets/textures/current/");

    // Read in zip file and recursively extract to opt dir
    for idx in 0..archive.len() {
        let mut extract = archive.by_index(idx).unwrap();

        let output_path = match extract.enclosed_name() {
            Some(path) => root.join(path.to_owned()),
            None => continue
        };

        if (*extract.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", idx, output_path.display());
            fs::create_dir_all(&output_path).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                idx,
                output_path.display(),
                extract.size()
            );
            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = File::create(&output_path).unwrap();
            io::copy(&mut extract, &mut outfile).unwrap();
        }
    }

    println!("File extract time: {} secs", begin.elapsed().unwrap().as_secs_f32())
}