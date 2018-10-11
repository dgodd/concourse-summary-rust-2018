#[macro_use] extern crate serde_derive;
use std::{
    error::Error,
};
use colored::*;
mod fly;

fn main() -> Result<(), Box<dyn Error>> {
    // http://ci.pcfdev.io/api/v1/info ;; pipelines ;; resources ;; userinfor ;; jobs

    let targets = fly::get_rc()?;
    for (name, host, _token) in &targets {
        println!("{} {}", "-->".red(), name.blue().bold());
        let pipelines = fly::get_pipelines(host)?;
        for pipeline in &pipelines {
            print!("    {}:", pipeline.name.bold());
            // paused_job aborted errored failed succeeded
            if let Some(num) = pipeline.statuses.get("succeeded") {
                print!("{}", format!(" Success: {}/{}", num, pipeline.num_jobs).green());
            }
            if let Some(num) = pipeline.statuses.get("failed") {
                print!("{}", format!(" Fail: {}/{}", num, pipeline.num_jobs).red());
            }
            let others : Vec<&String> = pipeline.statuses.iter().map(|(k,_)| k).filter(|k| *k != "succeeded" && *k != "failed").collect();
            if others.len() > 0 {
                print!(" {:?}", others);
            }
            print!("\n");
        }
    }

    Ok(())
}
