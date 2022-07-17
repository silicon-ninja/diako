use serde::Deserialize;



#[derive(Deserialize)]
pub struct KangModel {
    client: String,
    name: String,
    email: String,
    mode: String,
    key_path: String,
    location: Vec<String>,
}

