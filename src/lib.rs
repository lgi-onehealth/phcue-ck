use clap::Parser;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::exit;

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
    pub accession: String,
    reads: Vec<Reads>,
}

impl Run {
    /// Clean single end reads if there are paired end reads too
    /// This is if the user does not wish to have the single end reads, and
    /// keep only the paired end reads
    pub fn clean_single_end(&mut self) {
        if self.reads.len() == 3 {
            self.reads.remove(0);
        }
    }
}

pub enum OutputFormat {
    Json,
    Csv
}

/// Here, we implement the From trait for the Run struct, so that Run instances
/// can be derived from instances of the ENAApiResponse type.
/// Full example here: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6d15ef7f0834dae23b1bcea336c627f2
impl From<ENAApiResponse> for Run {
    fn from(response: ENAApiResponse) -> Self {
        let fastq_ftp_array = response.fastq_ftp.split(';').collect::<Vec<&str>>();
        let fastq_bytes_array = response.fastq_bytes.split(';').collect::<Vec<&str>>();
        let fastq_md5_array = response.fastq_md5.split(';').collect::<Vec<&str>>();
        let mut reads: Vec<Reads> = Vec::new();
        for i in 0..fastq_ftp_array.len() {
            reads.push(Reads {
                url: format!(
                    "ftp://{address}",
                    address = fastq_ftp_array[i].to_string().to_owned()
                ),
                bytes: match fastq_bytes_array[i].parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!(
                            "Could not parse {} as a number of bytes for accession {}",
                            fastq_bytes_array[i], response.run_accession
                        );
                        0
                    }
                },
                md5: fastq_md5_array[i].to_string().to_owned(),
            });
        }
        Self {
            accession: response.run_accession,
            reads,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    futures::stream::iter({
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
    .flatten()
    .collect::<Vec<Run>>()
}

/// CLI options and arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser, multiple = true, validator = validate_accession, required_unless_present = "file")]
    /// The accession of the run to query (must be an SRR, ERR or DRR accession)
    pub accession: Vec<String>,

    #[clap(
        short = 'n',
        long = "num-requests",
        value_name = "NUM",
        default_value = "1",
        help = "Maximum number of concurrent requests to make to the ENA API (max of 10 are allowed)"
    )]
    /// The maximum number of concurrent requests to make to the ENA API
    /// Default: 1
    /// Maximum: 10
    /// Minimum: 1
    pub num_requests: u8,

    #[clap(
        short,
        long,
        value_name = "FILE",
        help = "File containing accessions to query",
        required_unless_present = "accession"
    )]
    /// The file containing accessions to query
    /// If this is specified, the accessions will be read from this file
    /// If this is not specified, the accessions will be read from the command line
    pub file: Option<PathBuf>,

    #[clap(
        short,
        long,
        help = "Keep single end reads if there are paired end reads too"
    )]
    /// Keep single end reads if there are paired end reads too
    /// By default, we discard single end reads if there are paired end reads too.
    /// This is if the user does wish to have the single end reads
    pub keep_single_end: bool,

    #[clap(
        short = 'o',
        long = "output-format",
        value_name = "FORMAT",
        default_value = "json",
        help = "Format for output of data."
    )]
    /// The ourput format for the download links
    /// If this is specified, the data will be written to the output format
    /// If this is not specified, the data will be written to stdout
    pub format: String,
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
///
pub fn check_num_requests(num_requests: u8) -> usize {
    if num_requests > 10 {
        eprintln!("To be nice to ENA, we only allow up to 10 concurrent requests. Setting number of requests to 10.");
        10
    } else if num_requests < 1 {
        eprintln!("Number of requests should be at least 1. Setting number of requests to 1.");
        1
    } else {
        num_requests as usize
    }
}

/// Validate the output format to make sure it is one of 'json' or 'csv'. 
/// Case is ignored. 
pub fn check_output_format(format: String) -> OutputFormat {
    let f = format.to_lowercase();
    match f.as_ref() {
       "json" => OutputFormat::Json,
       "csv" => OutputFormat::Csv,
       _ => {
        eprintln!("Option {} unrecognized. Setting output format to JSON.", format);
        OutputFormat::Json
       } 
    }
    // if f.eq("json") || format.eq("csv") {
    //     format
    // } else {
    //     eprintln!("Option {} unrecognized. Setting output format to JSON.", format);
    //     "json"
    // }
}

/// A function to read accessions from a file and return a vector of validated
/// accessions. The function skips any empty lines, and will issue a warning
/// if it encounters an invalid accession. This deals with any potential header
/// lines in the file.
pub fn read_accessions(file: &PathBuf) -> Vec<String> {
    let file = match File::open(file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            exit(1);
        }
    };
    let reader = BufReader::new(file);
    reader
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .filter_map(|line| if line.is_empty() { None } else { Some(line) })
        .filter_map(|line| match validate_accession(line.as_str()) {
            Ok(_) => Some(line),
            Err(e) => {
                eprintln!("Error validating accession: {}. Ignoring this value...", e);
                None
            }
        })
        .collect()
}

pub fn print_csv(runs: Vec<Run>) -> Result<(), std::io::Error>{
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&["accession", "url", "md5", "bytes"])?;
    for run in runs {
        let accession = &run.accession;
        for read in run.reads {
            wtr.write_record(&[accession, &read.url, &read.md5, &read.bytes.to_string()])?;
        }
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_srr_accession() {
        let accession = "SRR1234567";
        let result = validate_accession(accession);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_err_accession() {
        let accession = "ERR1234567";
        let result = validate_accession(accession);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_drr_accession() {
        let accession = "DRR1234567";
        let result = validate_accession(accession);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_accession() {
        let accession = "1234567";
        let result = validate_accession(accession);
        assert!(result.is_err());
    }

    #[test]
    fn test_check_num_requests_valid() {
        let num_requests = 5;
        let result = check_num_requests(num_requests);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_check_num_requests_invalid_less_than_1() {
        let num_requests = 0;
        let result = check_num_requests(num_requests);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_check_num_requests_invalid_greater_than_10() {
        let num_requests = 11;
        let result = check_num_requests(num_requests);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_check_output_format_valid() {
        let format = String::from("json");
        let result = check_output_format(format);
        assert!(matches!(result, OutputFormat::Json));
        let format = String::from("JsOn");
        let result = check_output_format(format);
        assert!(matches!(result, OutputFormat::Json));
        let format = String::from("csv");
        let result = check_output_format(format);
        assert!(matches!(result, OutputFormat::Csv));
        let format = String::from("cSv");
        let result = check_output_format(format);
        assert!(matches!(result, OutputFormat::Csv));
    }

    #[test]
    fn test_check_output_format_default() {
        let format = String::from("foo");
        let result = check_output_format(format);
        assert!(matches!(result, OutputFormat::Json));
    }

    #[test]
    fn test_removal_single_reads() {
        let read_se = Reads {
            url: "read.fastq.gz".to_string(),
            md5: "md5".to_string(),
            bytes: 123,
        };
        let read_pe_1 = Reads {
            url: "read_1.fastq.gz".to_string(),
            md5: "md5".to_string(),
            bytes: 123,
        };
        let read_pe_2 = Reads {
            url: "read_2.fastq.gz".to_string(),
            md5: "md5".to_string(),
            bytes: 123,
        };
        let reads_se = vec![read_se.clone()];
        let reads_pe = vec![read_pe_1.clone(), read_pe_2.clone()];
        let reads_pe_se = vec![read_se.clone(), read_pe_1.clone(), read_pe_2.clone()];
        let run_se = Run {
            accession: "SRR1234567".to_string(),
            reads: reads_se,
        };
        let run_pe = Run {
            accession: "SRR1234567".to_string(),
            reads: reads_pe,
        };
        let run_pe_se = Run {
            accession: "SRR1234567".to_string(),
            reads: reads_pe_se,
        };
        let mut runs = vec![run_se, run_pe, run_pe_se];
        runs.iter_mut().for_each(|run| run.clean_single_end());
        assert_eq!(runs[0].reads[0], read_se);
        assert_eq!(runs[1].reads[0], read_pe_1);
        assert_eq!(runs[1].reads[1], read_pe_2);
        assert_eq!(runs[2].reads[0], read_pe_1);
        assert_eq!(runs[2].reads[1], read_pe_2);
    }
}
