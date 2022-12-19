use serde::Deserialize;

#[derive(Deserialize)]
pub struct DropboxLinkResult {
    pub links: Vec<DropboxLink>,
}
impl DropboxLinkResult {
    pub fn public_link(&self) -> Option<String> {
        let matching_links = self
            .links
            .iter()
            .filter(|a| a.is_public())
            .collect::<Vec<&DropboxLink>>();
        if matching_links.len() == 0 {
            None
        } else {
            Some(matching_links[0].url.clone())
        }
    }
}

#[derive(Deserialize)]
pub struct DropboxLink {
    link_permissions: DropboxLinkPermissions,
    pub url: String,
}
impl DropboxLink {
    pub fn is_public(&self) -> bool {
        let public: Vec<&DropboxLinkAudienceOption> = self
            .link_permissions
            .audience_options
            .iter()
            .filter(|a| a.audience.tag == "public")
            .collect();
        if public.len() == 0 {
            false
        } else {
            return public[0].allowed;
        }
    }
}
#[derive(Deserialize)]
pub struct DropboxLinkPermissions {
    // allow_contents: bool,
    allow_download: bool,
    audience_options: Vec<DropboxLinkAudienceOption>,
}

#[derive(Deserialize)]
pub struct DropboxLinkAudienceOption {
    allowed: bool,
    audience: DropboxTag,
}

#[derive(Deserialize)]
pub struct DropboxTag {
    #[serde(alias = ".tag")]
    tag: String,
}