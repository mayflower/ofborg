use ofborg::worker;
use serde_json;


pub fn from(data: &Vec<u8>) -> Result<SecurityAdvisoryJob, serde_json::error::Error> {
    return serde_json::from_slice(&data);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityAdvisoryJob {
}
