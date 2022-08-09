#  `phcue-ck`: a command line tool to get FTP urls for FASTQ data on ENA

## Background

`phcue-ck` (pronounced "F-Q-Seek") is a simple tool to get FTP urls for FASTQ files on ENA.
We developed this tool because we were frustrated with the lack of a simple tool that would
return the FTP url for the FASTQ files on ENA given the run accession. Our aim was to develop 
a simple tool that would take an accession (e.g., SRR123456) and would return the URL(s) for 
downloading the FASTQ from ENA's FTP server. We wanted a tool that could be easily used in 
Nextflow pipelines without complicated configurations or dependencies.

There are other tools out there that provide similar functionality, but we thought were too 
complicated to add to a workflow (e.g., as a workflow developed in Nextflow) or did not 
work consistently across a variety of accessions. 

Other tools you may wist to test are:
* `sra-tools`: https://github.com/ncbi/sra-tools
* `enaBrowserTools`: https://github.com/enasequence/enaBrowserTools
* `ffq`: https://github.com/pachterlab/ffq
* `pysradb`: https://github.com/saketkc/pysradb 

## Installation

## Usage

`phcue-ck` has a fairly simple interface.

You can get help on the command line by using the `-h/--help` flag.

```bash
$ phcue-ck --help
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
    -V, --version                     Print version information
```


The most simple case is you have an accession, you can run the following:

```bash
$ phcue-ck --accession SRR16298173
Querying ENA for accession: SRR16298173
[
  {
    "accession": "SRR16298173",
    "reads": [
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/073/SRR16298173/SRR16298173_1.fastq.gz",
        "md5": "76c841d58a4949736555f6fe2adcc86a",
        "bytes": 7332259
      },
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/073/SRR16298173/SRR16298173_2.fastq.gz",
        "md5": "861e40962c89d62bf298fde8ca1b7415",
        "bytes": 7765784
      }
    ]
  }
]

```

You can specify multiple accessions at once, and the output will be a list of results for each accession.

```bash
$ phcue-ck --accession SRR16298173 SRR16298174
Querying ENA for accession: SRR16298173
Querying ENA for accession: SRR16298174
[
  {
    "accession": "SRR16298173",
    "reads": [
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/073/SRR16298173/SRR16298173_1.fastq.gz",
        "md5": "76c841d58a4949736555f6fe2adcc86a",
        "bytes": 7332259
      },
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/073/SRR16298173/SRR16298173_2.fastq.gz",
        "md5": "861e40962c89d62bf298fde8ca1b7415",
        "bytes": 7765784
      }
    ]
  },
  {
    "accession": "SRR16298174",
    "reads": [
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/074/SRR16298174/SRR16298174_1.fastq.gz",
        "md5": "ca4365343d144947b5acf6e8ee124e49",
        "bytes": 7444532
      },
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/074/SRR16298174/SRR16298174_2.fastq.gz",
        "md5": "39523f0e9757e953cb0a5d707b9e2b58",
        "bytes": 10960575
      }
    ]
  }
]

```

If you have a list of accession in a file, you can run the following:

```bash
$ cat accessions.txt
SRR16298173
SRR16298174

$ phcue-ck -n2 --file accessions.txt
Querying ENA for accession: SRR16298173
Querying ENA for accession: SRR16298174
[
  {
    "accession": "SRR16298173",
    "reads": [
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/073/SRR16298173/SRR16298173_1.fastq.gz",
        "md5": "76c841d58a4949736555f6fe2adcc86a",
        "bytes": 7332259
      },
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/073/SRR16298173/SRR16298173_2.fastq.gz",
        "md5": "861e40962c89d62bf298fde8ca1b7415",
        "bytes": 7765784
      }
    ]
  },
  {
    "accession": "SRR16298174",
    "reads": [
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/074/SRR16298174/SRR16298174_1.fastq.gz",
        "md5": "ca4365343d144947b5acf6e8ee124e49",
        "bytes": 7444532
      },
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR162/074/SRR16298174/SRR16298174_2.fastq.gz",
        "md5": "39523f0e9757e953cb0a5d707b9e2b58",
        "bytes": 10960575
      }
    ]
  }
]

```

In the previous example, we used the `-n2` option to run two queries in parallel. The `-n` or `--num-requests` option can be used to set the number of concurrent requests to run at once. The default is 1 and the maximum is 10. _We chose 10 as the maximum to be nice to the ENA servers_.

Sometimes, an accession can have up to three files associated with it, the paired-end reads and, typically, a much smaller single-end reads file. By default, this single-end file is ignored. You can keep it in the output by using the `-k` or `--keep-single-end` option.

```bash
$ phcue-ck -k --accession ERR5556343
Querying ENA for accession: ERR5556343
[
  {
    "accession": "ERR5556343",
    "reads": [
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/ERR555/003/ERR5556343/ERR5556343.fastq.gz",
        "md5": "2b1b1d16d7b5a3d9c27f057c5064dd04",
        "bytes": 41148
      },
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/ERR555/003/ERR5556343/ERR5556343_1.fastq.gz",
        "md5": "2dd162ca91d340667b611d7f014eaaa5",
        "bytes": 7479353
      },
      {
        "url": "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/ERR555/003/ERR5556343/ERR5556343_2.fastq.gz",
        "md5": "8041deb0614dc669a3f28c20b330a599",
        "bytes": 8349710
      }
    ]
  }
]

```