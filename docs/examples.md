
## Single accession on the command-line

The most simple case is you have an accession, you can run the following:

```bash
phcue-ck --accession SRR16298173
```

```
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

## Multiple accessions on the command-line

You can specify multiple accessions at once, and the output will be a list of results for each accession.

```bash
phcue-ck --accession SRR16298173 SRR16298174
```

```
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

## Multiple accessions from a file

If you have a list of accession in a file, you can run the following:

```bash
cat accessions.txt
```
```bash
SRR16298173
SRR16298174
```

```bash
phcue-ck --file accessions.txt
```

```
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

## Running multiple queries in parallel

When running more than one query, you can set the the number of concurrent queries with the `-n/--num-requests` flag. 

For example, in the above example, we could've used  `-n 2` to run both queries concurrently.

The `-n` or `--num-requests` option can be used to set the number of concurrent requests to run at once. The default is 1 and the maximum is 10. _We chose 10 as the maximum to be nice to the ENA servers_.

```
phcue-ck -n2 --file accessions.txt
```

## Keeping single-end FASTQ when paired-end FASTQ is available

Sometimes, an accession can have up to three files associated with it, the paired-end reads and, typically, a much smaller, single-end reads file.

By default, this single-end file is ignored. You can keep it in the output by using the `-k` or `--keep-single-end` option.

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