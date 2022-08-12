# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2022-08-12

### Added 

- Dependency on openssl-probe to ensure system certicates are found
- Changelog

### Misc
- Cleaned up code following linting with clippy

## [0.1.1] - 2022-08-11

### Added

- Explicit version dependency on openssl using the vendored feature to avoid a dependency on the system openssl
- Dockerfile
- Better docs with mkdocs

## [0.1.0] - 2017-06-20

### Added

- Output FTP links for one or more accessions provided on the command line
- Output the FTP links for one or more accessions provided in a text file
- Allow for concurrent queries to the ENA API
- By default filter out single-end reads when paired-end reads present, but allow the user to keep the single-end read if they wish0