`phcue-ck` has a fairly simple interface.

You can get help on the command line by using the `-h/--help` flag.

```bash
phcue-ck --help
```

```bash
phcue-ck 0.1.0
Anders Goncalves da Silva <andersgs@gmail.com>
phcue-ck is a command line tool to obtain FTP links to FASTQ files from ENA using run accession

USAGE:
    phcue-ck [OPTIONS]

OPTIONS:
    -a, --accession <ACCESSION>...    The accession of the run to query (must be an SRR, ERR or DRR
                                      accession)
    -f, --file <FILE>                 File containing accessions to query
    -h, --help                        Print help information
    -k, --keep-single-end             Keep single end reads if there are paired end reads too
    -n, --num-requests <NUM>          Maximum number of concurrent requests to make to the ENA API
                                      (max of 10 are allowed) [default: 1]
    -o, --output-format <FORMAT>      Format for output of data. [default: json] [possible values:
                                      json, csv, csv-long]
    -V, --version                     Print version information
```
