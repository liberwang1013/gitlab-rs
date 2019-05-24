use Gitlab;
use UserId;

use error::*;
use std::borrow::Borrow;

#[derive(Serialize, Deserialize)]
pub struct SshKey {
    pub id: u32,
    pub title: String,
    pub key: String,
    pub created_at: String
}

impl Gitlab {
    pub fn list_ssh_keys(&self, id: UserId) -> Result<Vec<SshKey>> {
        let uri = format!("users/{}/keys", id);
        debug!("uri is {}", &uri);
        self.get_paged(&uri)
    }
}