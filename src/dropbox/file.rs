use crate::auth::DropboxAuthentication;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};
use ureq::serde_json::Value;

use super::links::DropboxLinkResult;

pub struct DropboxFile {
    path: String,
    filename: String,
    contents: Vec<u8>,
}
impl DropboxFile {
    pub fn download(path: &String, auth: &DropboxAuthentication) -> Option<DropboxFile> {
        let complete_path = DropboxFile::resolve_dropbox_path(path);
        let req = ureq::post("https://content.dropboxapi.com/2/files/download")
            .set("Authorization", &format!("Bearer {}", auth.key))
            .set(
                "Dropbox-API-Arg",
                &format!("{{\"path\":\"{}\"}}", complete_path),
            )
            .set("Content-Type", "application/octet-stream")
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
        let complete_path = DropboxFile::resolve_dropbox_path(path);
        let req = ureq::post("https://content.dropboxapi.com/2/files/download_zip")
            .set("Authorization", &format!("Bearer {}", auth.key))
            .set(
                "Dropbox-API-Arg",
                &format!("{{\"path\":\"{}\"}}", complete_path),
            )
            .set("Content-Type", "application/octet-stream")
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

    pub fn local(path: &String) -> Option<DropboxFile> {
        if let Ok(contents) = fs::read(path) {
            let path_buf = PathBuf::from(path);
            let path_file_name = path_buf.file_name().unwrap().to_str().unwrap();
            return Some(DropboxFile {
                path: path.clone(),
                filename: String::from(path_file_name),
                contents,
            });
        }
        None
    }

    pub fn upload(&self, auth: &DropboxAuthentication) -> Option<String> {
        let mut complete_path = String::new();
        if self.path.starts_with("/") {
            complete_path += "/";
            complete_path += self.filename();
        } else {
            complete_path += "/";
            complete_path += self.path();
        }
        let res = ureq::post("https://content.dropboxapi.com/2/files/upload")
            .set("Authorization", &format!("Bearer {}", auth.key))
            .set(
                "Dropbox-API-Arg",
                &format!("{{\"path\":\"{}\",\"mode\":\"overwrite\"}}", complete_path),
            )
            .set("Content-Type", "application/octet-stream")
            .send_bytes(self.data_ref());
        return if res.is_ok() {
            Some(complete_path)
        } else {
            None
        };
    }

    pub fn public_view_link(path: &String, auth: &DropboxAuthentication) -> Option<String> {
        let complete_path = DropboxFile::resolve_dropbox_path(path);
        if let Some(links) = DropboxFile::get_links(&complete_path, auth) {
            let target_link = links.public_link().unwrap_or(String::new());
            if target_link.is_empty() {
                println!("Link is empty, create one");
                return DropboxFile::create_public_view_link(&complete_path, auth);
            } else {
                println!("Using existing link");
                return Some(target_link);
            }
        }
        None
    }
    pub fn remove_all_links(path: &String, auth: &DropboxAuthentication) -> bool {
        let complete_path = DropboxFile::resolve_dropbox_path(path);
        if let Some(links) = DropboxFile::get_links(&complete_path, auth) {
            for l in links.links {
                let url = l.url;

                let res = ureq::post("https://api.dropboxapi.com/2/sharing/revoke_shared_link")
                    .set("Authorization", &format!("Bearer {}", auth.key))
                    .send_json(ureq::json!({
                        "url": url
                    }));
                if !res.is_ok() {
                    return false;
                }
            }
        }
        return true
    }

    fn get_links(path: &String, auth: &DropboxAuthentication) -> Option<DropboxLinkResult> {
        let req = ureq::post("https://api.dropboxapi.com/2/sharing/list_shared_links")
            .set("Authorization", &format!("Bearer {}", auth.key))
            .send_json(ureq::json!({ "path": path }));

        match req {
            Ok(res) => {
                let links: DropboxLinkResult = res.into_json().unwrap();
                return Some(links)
            }
            Err(_) => None,
        }
    }
    fn create_public_view_link(path: &String, auth: &DropboxAuthentication) -> Option<String> {
        let req =
            ureq::post("https://api.dropboxapi.com/2/sharing/create_shared_link_with_settings")
                .set("Authorization", &format!("Bearer {}", auth.key))
                .send_json(ureq::json!({
                    "path": path,
                    "settings": {
                        "access": "viewer",
                        "audience": "public",
                        "requested_visibility": "public"
                    }
                }));
        match req {
            Ok(res) => {
                let json: Value = res.into_json().unwrap();
                Some(json.get("url").unwrap().to_string())
            }
            Err(_) => None,
        }
    }

    pub fn path(&self) -> &String {
        &self.path
    }
    pub fn filename(&self) -> &String {
        &self.filename
    }
    pub fn data(self) -> Vec<u8> {
        self.contents
    }
    pub fn data_ref(&self) -> &[u8] {
        &self.contents
    }

    pub fn write(&self, path: &String) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        _ = file.write_all(&self.contents);
    }

    pub fn resolve_dropbox_path(path: &String) -> String {
        let mut output = String::new();
        if !path.starts_with("/") {
            output += "/";
        }
        output += path;
        return output;
    }
}
