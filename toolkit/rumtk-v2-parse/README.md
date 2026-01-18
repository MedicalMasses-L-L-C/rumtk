# rumtk-v2-parse

[![Build Status](https://github.com/kiseitai3/rumtk/actions/workflows/check.yml/badge.svg)](https://github.com/kiseitai3/rumtk/actions/workflows/check.yml) [![Crates.io](https://img.shields.io/crates/l/rumtk-hl7-v2-parse)](LICENSE-GPL3) [![Crates.io](https://img.shields.io/crates/v/rumtk-hl7-v2-parse)](https://crates.io/crates/rumtk-hl7-v2-parse) [![Released API docs](https://docs.rs/rumtk-hl7-v2-parse/badge.svg)](https://docs.rs/rumtk-hl7-v2-parse) [![Maintenance](https://img.shields.io/maintenance/yes/2026)](https://github.com/kiseitai3/rumtk)

Using RUMTK, this is a utility that implements the steps for parsing HL7 v2 messages.

# Goal

+ To provide a basic v2 parser utility kto allow for processing of messages into a searcheable format.
+ Have a utility that can be used on the terminal as part of other projects or a more complex pipeline.
+ Fully comply with the HL7 V2 standard in terms of parsing messages .

# Features

- [ ] HL7 v2 Parser
    - [x] Basic parsing of v2 message from pipes to `V2Message` type.
    - [x] Basic generation of v2 message from `V2Message` to pipes format.
    - [x] Allow reading of JSON or HL7 messages
    - [ ] Tests
    - [ ] Fuzz Targets

# Contributing

In its initial stages, I will be pushing code directly to the main branch. Once basic functionality has been stablished,
everyone including myself is required to open an issue for discussions, fork the project, and open a PR under your own
feature or main branch. I kindly ask you include a battery of unit tests with your PR to help protect the project
against regressions. Any contributions are very appreciated.
