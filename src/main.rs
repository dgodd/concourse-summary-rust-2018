#[macro_use]
extern crate serde_derive;
use colored::*;
use std::error::Error;
mod fly;

fn main() -> Result<(), Box<dyn Error>> {
    // http://ci.pcfdev.io/api/v1/info ;; pipelines ;; resources ;; userinfor ;; jobs

    let targets = fly::get_rc()?;
    for (name, host, _token) in &targets {
        println!("{} {}", "-->".red(), name.blue().bold());
        let pipelines = fly::get_pipelines(host)?;
        for pipeline in &pipelines {
            print!("    {}:", pipeline.name.bold());
            for (key, val) in &pipeline.statuses {
                match key {
                    // paused_job, aborted, errored
                    fly::Status::Success => {
                        print!("{}", format!(" {}%", 100 * val / pipeline.num_jobs).green())
                    }
                    fly::Status::Fail => {
                        print!("{}", format!(" {}%", 100 * val / pipeline.num_jobs).red())
                    }
                    _ => print!(
                        "{}",
                        format!(" {:?}:{}%", key, 100 * val / pipeline.num_jobs).yellow()
                    ),
                }
            }
            print!("\n");
        }
    }

    Ok(())
}
