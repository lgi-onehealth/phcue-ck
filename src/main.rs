use clap::Parser;
use reqwest::Error;
/// Get FTP address for FASTQ files given the accession number
///
/// {"run_accession":"SRR16298157","fastq_ftp":"ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_2.fastq.gz","fastq_bytes":"43409;42752","fastq_md5":"aaf5b365c1b45083c014baa35657b463;e80f09063bf017fa08b0dd881e840ed9","submitted_ftp":"","submitted_bytes":"","submitted_md5":"","sra_ftp":"ftp.sra.ebi.ac.uk/vol1/srr/SRR162/057/SRR16298157","sra_bytes":"157435","sra_md5":"baa98dd72f2a966be8f76569e46c03d9"}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Run {
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
    println!("{:?}", runs);
    Ok(())
}
