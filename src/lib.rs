use clap::Parser;
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
pub async fn query_ena(
    accession: &String,
    client: &reqwest::Client,
) -> Result<Vec<Run>, reqwest::Error> {
    let request_url = format!("https://www.ebi.ac.uk/ena/portal/api/filereport?accession={accession}&result=read_run&format=json", accession = accession);
    let response = client.get(&request_url).send().await?;
    let runs: Vec<Run> = response.json().await?;
    Ok(runs)
}

/// CLI options and arguments
// TODO: validate accession to start with SRR or ERR or DRR
// TODO: add the option to read accesssions from a file (one per line)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub accession: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
