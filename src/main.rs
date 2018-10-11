use std::{
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    // http://ci.pcfdev.io/api/v1/info ;; pipelines ;; resources ;; userinfor ;; jobs

    let url = format!("http://ci.pcfdev.io/api/v1/jobs");
    let json: serde_json::Value = reqwest::get(&url)?.json()?;

    println!("JSON: {:?}", json);

    Ok(())
}
