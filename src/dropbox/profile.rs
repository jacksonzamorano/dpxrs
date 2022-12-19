use crate::auth::DropboxAuthentication;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DropboxProfile {
    pub name: DropboxName
}
impl DropboxProfile {
    pub fn from(auth: &DropboxAuthentication) -> Option<DropboxProfile> {
        let response = ureq::post("https://api.dropboxapi.com/2/users/get_current_account")
            .set("Authorization", &format!("Bearer {}", auth.key))
            .call();
        match response {
            Ok(val) => {
                let parsed:Result<DropboxProfile, std::io::Error> = val.into_json();
                return match parsed {
                    Ok(p) => Some(p),
                    Err(e) => {
                        dbg!(e);
                        None
                    }
                }
            },
            Err(er) => {dbg!(er); None}
        }
            
    }
}

#[derive(Debug, Deserialize)]
pub struct DropboxName {
    pub abbreviated_name: String,
    pub display_name: String,
    pub familiar_name: String,
    pub given_name: String,
    pub surname: String
}