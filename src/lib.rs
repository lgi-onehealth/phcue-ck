use clap::Parser;
use futures::StreamExt;
use regex;
use serde::{Deserialize, Serialize};

/// A struct to hold the data returned from the ENA API
#[derive(Serialize, Deserialize, Clone, Debug)]
struct ENAApiResponse {
    run_accession: String,
    fastq_ftp: String,
    fastq_bytes: String,
    fastq_md5: String,
    submitted_ftp: String,
    submitted_md5: String,
    submitted_bytes: String,
    sra_ftp: String,
    sra_bytes: String,
    sra_md5: String,
}

/// A struct to hold the parsed data from the ENA API and return it to the user
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "ENAApiResponse")]
pub struct Run {
    accession: String,
    reads: Vec<Reads>,
}

/// Here, we implement the From trait for the Run struct, so that Run instances
/// can be derived from instances of the ENAApiResponse type.
/// Full example here: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6d15ef7f0834dae23b1bcea336c627f2
impl From<ENAApiResponse> for Run {
    fn from(response: ENAApiResponse) -> Self {
        let fastq_ftp_array = response.fastq_ftp.split(";").collect::<Vec<&str>>();
        let fastq_bytes_array = response.fastq_bytes.split(";").collect::<Vec<&str>>();
        let fastq_md5_array = response.fastq_md5.split(";").collect::<Vec<&str>>();
        let mut reads: Vec<Reads> = Vec::new();
        for i in 0..fastq_ftp_array.len() {
            reads.push(Reads {
                url: format!(
                    "ftp://{address}",
                    address = fastq_ftp_array[i].to_string().to_owned()
                ),
                bytes: fastq_bytes_array[i].parse::<u32>().unwrap(),
                md5: fastq_md5_array[i].to_string().to_owned(),
            });
        }
        Self {
            accession: response.run_accession,
            reads,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Reads {
    url: String,
    md5: String,
    bytes: u32,
}

/// A function to query the ENA API and return a vector of Run instances
async fn query_ena(
    accession: &String,
    client: &reqwest::Client,
) -> Result<Vec<Run>, reqwest::Error> {
    let request_url = format!("https://www.ebi.ac.uk/ena/portal/api/filereport?accession={accession}&result=read_run&format=json", accession = accession);
    let response = client.get(&request_url).send().await?;
    let runs: Vec<Run> = response.json().await?;
    Ok(runs)
}

/// A function to query the ENA API and return a vector of Run instances
/// This function is used to query the ENA API concurrently across multiple accessions
pub async fn concurrent_query_ena(accessions: Vec<String>, num_requests: usize) -> Vec<Run> {
    let client = reqwest::Client::new();
    let nested_runs = futures::stream::iter({
        accessions.iter().map(|accession| {
            let client = client.clone();
            eprintln!("Querying ENA for accession: {}", accession);
            async move {
                match query_ena(accession, &client).await {
                    Ok(run) => Some(run),
                    Err(e) => {
                        eprintln!("Error querying ENA for accession: {}", accession);
                        eprintln!("Error: {}", e);
                        None
                    }
                }
            }
        })
    })
    .buffer_unordered(num_requests)
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .filter_map(|run| run)
    .collect::<Vec<_>>();
    let runs = nested_runs.into_iter().flatten().collect::<Vec<Run>>();
    runs
}

/// CLI options and arguments
// TODO: add the option to read accesssions from a file (one per line)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser, multiple = true, validator = validate_accession)]
    /// The accession of the run to query (must be an SRR, ERR or DRR accession)
    pub accession: Vec<String>,

    #[clap(
        short = 'n',
        long = "num-requests",
        value_name = "NUM",
        default_value = "1",
        help = "Maximum number of concurrent requests to make to the ENA API"
    )]
    /// The maximum number of concurrent requests to make to the ENA API
    /// Default: 1
    /// Maximum: 10
    /// Minimum: 1
    pub num_requests: u8,
}

pub fn parse_args() -> Args {
    Args::parse()
}

/// Validate the accession number to make sure it starts with SRR, ERR,
///  or DRR
fn validate_accession(accession: &str) -> Result<(), String> {
    let regex = regex::Regex::new(r"^(SRR|ERR|DRR)[0-9]{6,10}$").unwrap();
    if regex.is_match(accession) {
        Ok(())
    } else {
        Err(format!("{} is not a valid accession number", accession))
    }
}

/// Validate the total number of concurrent requests to make to the ENA API
/// to make sure it is within the bounds of 1 and 10. If not, return the minimum
/// if num_requests is less than 1 or maximum value if num_requests is larger than 10.
/// We have chosen to bound it to 10 to be nice to the ENA API.
pub fn check_num_requests(num_requests: u8) -> usize {
    if num_requests > 10 {
        eprintln!("To be nice to ENA, we only allow up to 10 concurrent requests. Setting number of requests to 10.");
        return 10;
    } else if num_requests < 1 {
        eprintln!("Number of requests should be at least 1. Setting number of requests to 1.");
        return 1;
    } else {
        return num_requests as usize;
    }
}
