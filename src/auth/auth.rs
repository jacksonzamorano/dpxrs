use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use ureq::serde_json;

pub struct DropboxAuthentication {
    ready: bool,
    verification: Vec<u8>,
    pub key: String,
}

impl DropboxAuthentication {
    pub fn create() -> DropboxAuthentication {
        let mut a = DropboxAuthentication {
            ready: false,
            verification: pkce::code_verifier(128),
            key: String::new(),
        };
        a.try_recall();
        return a;
    }

    pub fn authorized(&self) -> bool {
        return self.ready;
    }

    fn get_storage_path(&self) -> Option<String> {
        let p_dirs = ProjectDirs::from("com", "jacksonzamorano", "dpx")?;
        let as_dir = p_dirs.config_dir();
        _ = fs::create_dir(as_dir);
        let path = as_dir
            .join(PathBuf::from("dropbox.token.txt"))
            .to_str()
            .unwrap()
            .to_string();

        Some(path)
    }

    pub fn try_recall(&mut self) {
        if let Some(path) = self.get_storage_path() {
            if let Ok(contents) = fs::read_to_string(path) {
                self.key = contents.replace("\n", "");
                self.ready = true;
            } else {
                println!("Couldn't get contents");
            }
        } else {
            print!("couldn't get storage path")
        }
    }

    pub fn get_auth_link(&self) -> String {
        let key = "s3q3092azd1y0ne";
        return format!("https://www.dropbox.com/oauth2/authorize?response_type=token&client_id={}&redirect_uri=https://www.flareapplications.com/display", key);
    }

    pub fn set_code(&mut self, code: String) {
        self.key = code;
        self.ready = true;        
        if let Some(p) = self.get_storage_path() {
            _ = fs::write(p, self.key.as_bytes());
        }
    }

    pub fn logout(&mut self) {
        if let Some(p) = self.get_storage_path() {
            _ = fs::remove_file(p);
        }
        self.key = String::new();
        self.ready = false;
    }
}
