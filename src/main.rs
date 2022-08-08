/// Get FTP address for FASTQ files given the accession number
/// Example output from the API:
/// {"run_accession":"SRR16298157","fastq_ftp":"ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/057/SRR16298157/SRR16298157_2.fastq.gz","fastq_bytes":"43409;42752","fastq_md5":"aaf5b365c1b45083c014baa35657b463;e80f09063bf017fa08b0dd881e840ed9","submitted_ftp":"","submitted_bytes":"","submitted_md5":"","sra_ftp":"ftp.sra.ebi.ac.uk/vol1/srr/SRR162/057/SRR16298157","sra_bytes":"157435","sra_md5":"baa98dd72f2a966be8f76569e46c03d9"}
use fq_ck::{parse_args, query_ena, Run};
use futures::StreamExt;
use reqwest::Error;
// TODO: add option to limit the number of concurrent requests
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = parse_args();
    let client = reqwest::Client::new();
    let nested_runs = futures::stream::iter({
        args.accession.iter().map(|accession| {
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
    .buffer_unordered(2)
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .filter_map(|run| run)
    .collect::<Vec<_>>();
    let runs = nested_runs.into_iter().flatten().collect::<Vec<Run>>();
    println!("{}", serde_json::to_string_pretty(&runs).unwrap());
    Ok(())
}
