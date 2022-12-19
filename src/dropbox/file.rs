use std::{
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
};

use crate::auth::DropboxAuthentication;

pub struct DropboxFile {
    path: String,
    filename: String,
    contents: Vec<u8>,
}

impl DropboxFile {
    pub fn download(path: &String, auth: &DropboxAuthentication) -> Option<DropboxFile> {
        let req = ureq::post("https://content.dropboxapi.com/2/files/download")
            .set("Authorization", &format!("Bearer {}", auth.key))
            .set("Dropbox-API-Arg", &format!("{{\"path\":\"{}\"}}", path))
            .call();
        match req {
            Ok(val) => {
                let mut reader = val.into_reader();
                let mut buf: Vec<u8> = Vec::new();
                let _ = reader.read_to_end(&mut buf);
                let write_path = PathBuf::from(path);
                let write_path_name = write_path.file_name().unwrap();
                Some(DropboxFile {
                    path: path.clone(),
                    filename: String::from(write_path_name.to_str().unwrap()),
                    contents: buf,
                })
            }
            Err(_e) => None,
        }
    }

    pub fn download_zip(path: &String, auth: &DropboxAuthentication) -> Option<DropboxFile> {
        let req = ureq::post("https://content.dropboxapi.com/2/files/download_zip")
            .set("Authorization", &format!("Bearer {}", auth.key))
            .set("Dropbox-API-Arg", &format!("{{\"path\":\"{}\"}}", path))
            .call();
        match req {
            Ok(val) => {
                let mut reader = val.into_reader();
                let mut buf: Vec<u8> = Vec::new();
                let _ = reader.read_to_end(&mut buf);
                let write_path = PathBuf::from(path);
                let write_path_name = write_path.file_name().unwrap();
                Some(DropboxFile {
                    path: path.clone(),
                    filename: format!("{}.zip", String::from(write_path_name.to_str().unwrap())),
                    contents: buf,
                })
            }
            Err(_e) => None,
        }
    }

    pub fn path(&self) -> &String { &self.path }

    pub fn write(&self, path: &String) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.filename)
            .unwrap();
        _ = file.write_all(&self.contents);
    }
}
