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
    fastq_ftp: Option<String>,
    fastq_1_ftp: Option<String>,
    fastq_2_ftp: Option<String>,
    fastq_bytes: Option<u32>,
    fastq_1_bytes: Option<u32>,
    fastq_2_bytes: Option<u32>,
    fastq_md5: Option<String>,
    fastq_1_md5: Option<String>,
    fastq_2_md5: Option<String>,
}

/// Here, we implement the From trait for the Run struct, so that Run instances
/// can be derived from instances of the ENAApiResponse type.
/// Full example here: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6d15ef7f0834dae23b1bcea336c627f2
impl From<ENAApiResponse> for Run {
    fn from(response: ENAApiResponse) -> Self {
        let fastq_ftp_array = response.fastq_ftp.split(";").collect::<Vec<&str>>();
        let fastq_bytes_array = response.fastq_bytes.split(";").collect::<Vec<&str>>();
        let fastq_md5_array = response.fastq_md5.split(";").collect::<Vec<&str>>();
        let mut fastq_ftp = None;
        let mut fastq_1_ftp = None;
        let mut fastq_2_ftp = None;
        let mut fastq_bytes = None;
        let mut fastq_1_bytes = None;
        let mut fastq_2_bytes = None;
        let mut fastq_md5 = None;
        let mut fastq_1_md5 = None;
        let mut fastq_2_md5 = None;
        // Three cases, when there is a single FASTQ file,
        // when there are three FASTQ files (the paired files with suffix
        // 1 and 2, and a third one usually with few
        // reads and no suffix), and the case when there are only two, paired,
        //  FASTQ files (1 and 2).
        if fastq_ftp_array.len() == 1 {
            fastq_ftp = Some(fastq_ftp_array[0].to_string());
            fastq_bytes = Some(fastq_bytes_array[0].parse::<u32>().unwrap());
            fastq_md5 = Some(fastq_md5_array[0].to_string());
        } else if fastq_ftp_array.len() == 3 {
            fastq_ftp = Some(fastq_ftp_array[0].to_string());
            fastq_1_ftp = Some(fastq_ftp_array[1].to_string());
            fastq_2_ftp = Some(fastq_ftp_array[2].to_string());
            fastq_bytes = Some(fastq_bytes_array[0].parse::<u32>().unwrap());
            fastq_1_bytes = Some(fastq_bytes_array[1].parse::<u32>().unwrap());
            fastq_2_bytes = Some(fastq_bytes_array[2].parse::<u32>().unwrap());
            fastq_md5 = Some(fastq_md5_array[0].to_string());
            fastq_1_md5 = Some(fastq_md5_array[1].to_string());
            fastq_2_md5 = Some(fastq_md5_array[2].to_string());
        } else {
            fastq_1_ftp = Some(fastq_ftp_array[0].to_string());
            fastq_2_ftp = Some(fastq_ftp_array[1].to_string());
            fastq_1_bytes = Some(fastq_bytes_array[0].parse::<u32>().unwrap());
            fastq_2_bytes = Some(fastq_bytes_array[1].parse::<u32>().unwrap());
            fastq_1_md5 = Some(fastq_md5_array[0].to_string());
            fastq_2_md5 = Some(fastq_md5_array[1].to_string());
        }
        Self {
            accession: response.run_accession,
            fastq_ftp,
            fastq_1_ftp,
            fastq_2_ftp,
            fastq_bytes,
            fastq_1_bytes,
            fastq_2_bytes,
            fastq_md5,
            fastq_1_md5,
            fastq_2_md5,
        }
    }
}

/// CLI options and arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub accession: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
