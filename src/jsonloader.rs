use std::collections::HashMap;
use std::fs::File;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthUser {
    pub id: u64,
    pub wnam: String,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Auth {
    _default: HashMap<u64, AuthUser>,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth: Auth = serde_json::from_reader(File::open("auth.json")?)?;
    println!("suc");
    let mut auth2 = auth._default.into_iter().collect::<Vec<_>>();
    auth2.sort_by_key(|(x, _)| *x);
    let auth3 = auth2.into_iter().map(|(_, u)| u).collect::<Vec<_>>();
    serde_json::to_writer(File::create("auth2.json")?, &*auth3)?;

    Ok(())
}