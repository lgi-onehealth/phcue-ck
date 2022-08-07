/// Get FTP address for FASTQ files given the accession number
/// Example output from the API:
/// {"run_accession":"SRR16298157","fastq_ftp":"ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_2.fastq.gz","fastq_bytes":"43409;42752","fastq_md5":"aaf5b365c1b45083c014baa35657b463;e80f09063bf017fa08b0dd881e840ed9","submitted_ftp":"","submitted_bytes":"","submitted_md5":"","sra_ftp":"ftp.sra.ebi.ac.uk/vol1/srr/SRR162/057/SRR16298157","sra_bytes":"157435","sra_md5":"baa98dd72f2a966be8f76569e46c03d9"}
use clap::Parser;
use reqwest::Error;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "ENAApiResponse")]
struct Run {
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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    accession: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let request_url = format!("https://www.ebi.ac.uk/ena/portal/api/filereport?accession={accession}&result=read_run&format=json", accession = args.accession);
    eprintln!("{}", request_url);
    let response = reqwest::get(&request_url).await?;
    let runs: Vec<Run> = response.json().await?;
    println!("{}", serde_json::to_string_pretty(&runs).unwrap());
    Ok(())
}
