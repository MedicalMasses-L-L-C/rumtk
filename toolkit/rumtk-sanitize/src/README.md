# rumtk-sanitize

<a href="https://opencollective.com/medicalmasses-llc/projects/rumtk-v2" rel="Help finance the project!">
    <img src="https://opencollective.com/medicalmasses-llc/contribute/button@2x.png?color=blue" width=300 />
</a>

[![Build Status](https://github.com/kiseitai3/rumtk/actions/workflows/check.yml/badge.svg)](https://github.com/kiseitai3/rumtk/actions/workflows/check.yml) [![Crates.io](https://img.shields.io/crates/l/rumtk-sanitize)](LICENSE-GPL3) [![Crates.io](https://img.shields.io/crates/v/rumtk-sanitize)](https://crates.io/crates/rumtk-sanitize) [![Released API docs](https://docs.rs/rumtk-sanitize/badge.svg)](https://docs.rs/rumtk-sanitize) [![Maintenance](https://img.shields.io/maintenance/yes/2026)](https://github.com/kiseitai3/rumtk)

Using RUMTK, this is a utility that implements the steps for parsing HL7 v2 messages.

# Goal

+ To provide a basic v2 parser utility kto allow for processing of messages into a searchable format.
+ Have a utility that can be used on the terminal as part of other projects or a more complex pipeline.
+ Fully comply with the HL7 V2 standard in terms of parsing messages .

# Features

- [ ] Sanitizer
  - [x] HTML
  - [ ] ASCII Escape Sequences
  - [ ] HL7 v2

# Contributing

In its initial stages, I will be pushing code directly to the main branch. Once basic functionality has been stablished,
everyone including myself is required to open an issue for discussions, fork the project, and open a PR under your own
feature or main branch. I kindly ask you include a battery of unit tests with your PR to help protect the project
against regressions. Any contributions are very appreciated.
