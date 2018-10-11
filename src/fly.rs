use std::{collections::BTreeMap, env, error::Error, fs::File, io::prelude::Read, path::Path};

#[derive(Debug, Deserialize)]
struct Token {
    value: String,
}

#[derive(Debug, Deserialize)]
struct Target {
    api: String,
    token: Token,
}

#[derive(Debug, Deserialize)]
struct FlyRC {
    targets: BTreeMap<String, Target>,
}

pub fn get_rc() -> Result<Vec<(String, String, String)>, Box<dyn Error>> {
    let home = env::var("HOME")?;
    let mut f = File::open(Path::new(&home).join(".flyrc"))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let fly_rc: FlyRC = serde_yaml::from_str(&contents)?;
    let targets = fly_rc
        .targets
        .iter()
        .map(|(key, target)| {
            (
                key.to_owned(),
                target.api.to_owned(),
                target.token.value.to_owned(),
            )
        }).collect();
    Ok(targets)
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub name: String,
    pub num_jobs: u64,
    pub statuses: BTreeMap<Status, u64>,
}

#[derive(Debug, Deserialize)]
struct Job {
    pipeline_name: String,
    // team_name: String,
    finished_build: Option<Build>,
}
#[derive(Debug, Deserialize)]
struct Build {
    status: Status,
}
#[derive(Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Status {
    #[serde(rename = "paused_job")]
    PausedJob,
    #[serde(rename = "aborted")]
    Abort,
    #[serde(rename = "errored")]
    Error,
    #[serde(rename = "failed")]
    Fail,
    #[serde(rename = "succeeded")]
    Success,
}

pub fn get_pipelines(host: &str) -> Result<Vec<Pipeline>, Box<dyn Error>> {
    let url = format!("{}/api/v1/jobs", host);
    let json: Vec<Job> = reqwest::get(&url)?.json()?;

    let mut a = BTreeMap::new();
    for ref job in &json {
        let pipeline = a.entry(job.pipeline_name.to_owned()).or_insert(Pipeline {
            name: job.pipeline_name.to_owned(),
            num_jobs: 0,
            statuses: BTreeMap::new(),
        });
        pipeline.num_jobs += 1;
        match &job.finished_build {
            Some(fb) => {
                let status = &fb.status;
                *pipeline.statuses.entry(status.to_owned()).or_insert(0) += 1;
            }
            None => {}
        }
    }

    Ok(a.values().cloned().collect())
}
