
<img src="assets/phcue-ck.svg" width="100%">
<p align="center">
    <img src="https://github.com/lgi-onehealth/phcue-ck/actions/workflows/ci.yml/badge.svg" />
    <img alt="Crates.io" src="https://img.shields.io/crates/v/phcue-ck?color=light" />
    <img alt="Conda" src="https://img.shields.io/conda/v/bioconda/phcue-ck?color=light%20&logo=anaconda" />
    <img alt="Docker Image Version (latest semver)" src="https://img.shields.io/docker/v/lighthousegenomics/phcue-ck?color=light&label=dockerhub&logo=docker" />
    <img src="https://img.shields.io/github/license/lgi-onehealth/phcue-ck?color=light%20green" />
</p>

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

* `sra-tools`: [https://github.com/ncbi/sra-tools](https://github.com/ncbi/sra-tools)
* `enaBrowserTools`: [https://github.com/enasequence/enaBrowserTools](https://github.com/enasequence/enaBrowserTools)
* `ffq`: [https://github.com/pachterlab/ffq](https://github.com/pachterlab/ffq)
* `pysradb`: [https://github.com/saketkc/pysradb](https://github.com/saketkc/pysradb)


## Acknowledgements

The binoculars used in the logo are from __Icons by [svgrepo.com](svgrepo.com)__.