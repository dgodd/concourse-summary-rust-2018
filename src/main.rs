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
        println!("{} {}", "-->".red(), name.bold());
        let pipelines = fly::get_pipelines(host, token)?;
        for pipeline in &pipelines {
            let mut pipeline_name = pipeline.name.normal();
            if pipeline.paused {
                pipeline_name = pipeline_name.bold().blue();
            }
            print!("    {}:", pipeline_name);
            for (key, val) in &pipeline.statuses {
                let perc = format!("{}%", 100 * val / pipeline.num_jobs);
                print!(
                    " {}",
                    match key {
                        fly::Status::Success => perc.green(),
                        fly::Status::Fail => perc.red().bold(),
                        _ => format!("{:?}:{}%", key, perc).yellow(),
                    }
                )
            }
            print!("\n");
        }
    }

    Ok(())
}
