# Rumtk-Benchmark

<a href="https://opencollective.com/medicalmasses-llc/projects/rumtk-v2" rel="Help finance the project!">
    <img src="https://opencollective.com/medicalmasses-llc/contribute/button@2x.png?color=blue" width=300 />
</a>

[![Build Status](https://github.com/MedicalMasses-L-L-C/rumtk/actions/workflows/check.yml/badge.svg)](https://github.com/MedicalMasses-L-L-C/rumtk/actions/workflows/check.yml) [![Crates.io](https://img.shields.io/crates/l/rumtk-sanitize)](LICENSE-GPL3) [![Crates.io](https://img.shields.io/crates/v/rumtk-sanitize)](https://crates.io/crates/rumtk-sanitize) [![Released API docs](https://docs.rs/rumtk-sanitize/badge.svg)](https://docs.rs/rumtk-sanitize) [![Maintenance](https://img.shields.io/maintenance/yes/2026)](https://github.com/MedicalMasses-L-L-C/rumtk)

Rust's Universal Medical Toolkit is a toolkit being developed to put together a set of tools and libraries to facilitate
communication and automation in medicine.

This tool provides a way to filter (or sanitize) piped data in the form of a rumtk utility.

# Goal

+ To create a simple toolkit with the necessary libraries, dependencies, and utilities for managing bridging HL7 V2
  Medical IT infrastructure to FHIR based systems.
+ Also, I would like the project to be accessible to hospitals to enable interoperability between systems. I plan to
  package it for package managers and containers.
+ The toolkit will focus on increasing security and simplicity with the first step taken by starting the project using
  Rust.
+ The toolkit shall foster reliability and will make attempts to be as strictly standards compliant as possible.
  Strictness may be relaxed later once the project sees use in the wild.

# Features

- [ ] Sanitizer Utility
    - [x] HTML Fragment Sanitization
    - [ ] ASCII Escape Sequence Sanitization
    - [ ] HL7 V2 Message Sanitization
    - [ ] Tests

# Contributing

In its initial stages, I will be pushing code directly to the main branch. Once basic functionality has been stablished,
everyone including myself is required to open an issue for discussions, fork the project, and open a PR under your own
feature or main branch. I kindly ask you include a battery of unit tests with your PR to help protect the project
against regressions. Any contributions are very appreciated.
