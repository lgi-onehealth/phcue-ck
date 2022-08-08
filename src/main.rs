/// Get FTP address for FASTQ files given the accession number
/// Example output from the API:
/// {"run_accession":"SRR16298157","fastq_ftp":"ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_2.fastq.gz","fastq_bytes":"43409;42752","fastq_md5":"aaf5b365c1b45083c014baa35657b463;e80f09063bf017fa08b0dd881e840ed9","submitted_ftp":"","submitted_bytes":"","submitted_md5":"","sra_ftp":"ftp.sra.ebi.ac.uk/vol1/srr/SRR162/057/SRR16298157","sra_bytes":"157435","sra_md5":"baa98dd72f2a966be8f76569e46c03d9"}
use fq_ck::{parse_args, query_ena, Run};
use reqwest::Error;
// TODO: allow for multuple accessions to be passed as input and then async loop over them using the function above
// TODO: add option to limit the number of concurrent requests
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = parse_args();
    let client = reqwest::Client::new();
    let mut nested_runs: Vec<Vec<Run>> = vec![];
    for accession in args.accession {
        eprintln!("Querying ENA for accession {}", accession);
        let run: Vec<Run> = query_ena(&accession, &client).await?;
        nested_runs.push(run);
    }
    let runs: Vec<Run> = nested_runs.into_iter().flatten().collect();
    println!("{}", serde_json::to_string_pretty(&runs).unwrap());
    Ok(())
}
