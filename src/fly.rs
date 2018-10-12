use std::{collections::BTreeMap, error::Error, fs::File, io::prelude::Read, path::Path};

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

pub fn get_rc(home: &str) -> Result<Vec<(String, String, String)>, Box<dyn Error>> {
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

#[derive(Debug, Clone, Deserialize)]
pub struct Pipeline {
    pub id: u64,
    pub name: String,
    pub paused: bool,
    pub public: bool,
    pub team_name: String,
    #[serde(skip_deserializing)]
    pub num_jobs: u64,
    #[serde(skip_deserializing)]
    pub statuses: BTreeMap<Status, u64>,
}

#[derive(Debug, Deserialize)]
struct Job {
    pipeline_name: String,
    team_name: String,
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

pub fn get_pipelines(host: &str, token: &str) -> Result<Vec<Pipeline>, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut pipelines: Vec<Pipeline> = client
        .get(&format!("{}/api/v1/pipelines", host))
        .header("Authorization", format!("Bearer+{}", token))
        .send()?
        .json()?;
    let mut a = BTreeMap::new();
    for (i, pipeline) in pipelines.iter().enumerate() {
        a.insert((pipeline.team_name.to_owned(), pipeline.name.to_owned()), i);
    }

    let jobs: Vec<Job> = client
        .get(&format!("{}/api/v1/jobs", host))
        .header("Authorization", format!("Bearer+{}", token))
        .send()?
        .json()?;

    for ref job in &jobs {
        a.entry((job.team_name.to_owned(), job.pipeline_name.to_owned()))
            .and_modify(|i| {
                let ref mut pipeline = pipelines[*i];
                pipeline.num_jobs += 1;
                match &job.finished_build {
                    Some(fb) => {
                        let status = &fb.status;
                        *pipeline.statuses.entry(status.to_owned()).or_insert(0) += 1;
                    }
                    None => {}
                }
            });
    }

    Ok(pipelines)
}
