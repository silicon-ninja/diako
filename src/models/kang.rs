use serde::Deserialize;
#[derive(Deserialize)]
pub struct KangModel {
    client: String,
    name: String,
    email: String,
    mode: String,
    ssh_key: String,
    url : String,
    description: String,
    location_status: bool,
    location: Vec<KandModelLocation>,
    location_tag: String,
}


#[derive(Deserialize)]
pub struct KandModelLocation {
    name: String,
    lattitue: String,
    longitude: String,
}