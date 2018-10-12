#[macro_use]
extern crate serde_derive;
use colored::*;
use std::{env, error::Error};
mod fly;

fn main() -> Result<(), Box<dyn Error>> {
    // http://ci.pcfdev.io/api/v1/info ;; pipelines ;; resources ;; userinfor ;; jobs

    let targets = fly::get_rc(&env::var("HOME")?)?;
    for (name, host, token) in &targets {
        // if name != "ci" {
        //     continue;
        // }
        println!("{} {}", "-->".red(), name.blue().bold());
        let pipelines = fly::get_pipelines(host, token)?;
        for pipeline in &pipelines {
            print!("    {}:", pipeline.name.bold());
            for (key, val) in &pipeline.statuses {
                let perc = format!("{}%", 100 * val / pipeline.num_jobs);
                print!(
                    " {}",
                    match key {
                        fly::Status::Success => perc.green(),
                        fly::Status::Fail => perc.red(),
                        _ => format!("{:?}:{}%", key, perc).yellow(),
                    }
                )
            }
            print!("\n");
        }
    }

    Ok(())
}
