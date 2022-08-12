/// Get FTP address for FASTQ files given the accession number
/// Example output from the API:
/// {"run_accession":"SRR16298157","fastq_ftp":"ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_2.fastq.gz","fastq_bytes":"43409;42752","fastq_md5":"aaf5b365c1b45083c014baa35657b463;e80f09063bf017fa08b0dd881e840ed9","submitted_ftp":"","submitted_bytes":"","submitted_md5":"","sra_ftp":"ftp.sra.ebi.ac.uk/vol1/srr/SRR162/057/SRR16298157","sra_bytes":"157435","sra_md5":"baa98dd72f2a966be8f76569e46c03d9"}
use phcue_ck::{check_num_requests, concurrent_query_ena, parse_args, read_accessions, Run};
use reqwest::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    openssl_probe::init_ssl_cert_env_vars();
    let args = parse_args();
    let accessions = match args.file {
        Some(file) => read_accessions(&file),
        None => args.accession,
    };
    let num_requests = check_num_requests(args.num_requests);
    let mut runs: Vec<Run> = concurrent_query_ena(accessions, num_requests).await;
    if !runs.is_empty() {
        runs.sort_by(|a, b| a.accession.cmp(&b.accession));
        if !args.keep_single_end {
            runs.iter_mut().for_each(|run| run.clean_single_end());
        }
        println!("{}", serde_json::to_string_pretty(&runs).unwrap());
    }
    Ok(())
}
