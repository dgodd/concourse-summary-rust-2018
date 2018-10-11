#[macro_use] extern crate serde_derive;
use std::{
    error::Error,
};
mod fly;

fn main() -> Result<(), Box<dyn Error>> {
    // http://ci.pcfdev.io/api/v1/info ;; pipelines ;; resources ;; userinfor ;; jobs

    let targets = fly::get_rc()?;
    for (name, host, _token) in &targets {
        println!("-----> {}", name);
        let pipelines = fly::get_pipelines(host)?;
        for pipeline in &pipelines {
            println!("  PIPELINE: {:?}", pipeline);
        }
    }

    Ok(())
}
