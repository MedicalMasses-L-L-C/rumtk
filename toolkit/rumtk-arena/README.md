# Rumtk-arena

[![Build Status](https://github.com/kiseitai3/rumtk/actions/workflows/check.yml/badge.svg)](https://github.com/kiseitai3/rumtk/actions/workflows/check.yml) [![Crates.io](https://img.shields.io/crates/l/rumtk-arena)](COPYING.LESSER) [![Crates.io](https://img.shields.io/crates/v/rumtk-arena)](https://crates.io/crates/rumtk-arena) [![Released API docs](https://docs.rs/rumtk-arena/badge.svg)](https://docs.rs/rumtk-arena) [![Maintenance](https://img.shields.io/maintenance/yes/2026)](https://github.com/kiseitai3/rumtk)

Rust's Universal Medical Toolkit is a toolkit being developed to put together a set of tools and libraries to facilitate
communication and automation in medicine.

This crate provides a simple implementation of the Arena or Bump memory allocation data structure. This simple implementation 
should be very close to bare metal performance because all we are doing is tracking an offset counter from a base pointer.
Although unsafe blocks are used, we yield an error if allocation violates the size constraint.

Be aware that we have not established or worked towards multithreading safety. This data structure is meant to be used 
withing the scope of a single function. It is not meant to be exposed for allocations by multiple threads simultaneously.
If it becomes a real need, we will update the library accordingly.

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

- [ ] Toolkit Arena Allocator Library
    - [x] Arena Allocator.
    - [x] Arena-aware Collections.
    - [x] Generic type allocation.
    - [x] Strict failure on requesting more memory than pre-allocated.
    - [x] Implementation of `Allocator` API trait.
    - [x] Implementation of `Global Allocator` API trait.
    - [x] Compatibility with `std` (our collections is just a series of helpers wrapping the nightly std collections).

# Contributing

In its initial stages, I will be pushing code directly to the main branch. Once basic functionality has been stablished,
everyone including myself is required to open an issue for discussions, fork the project, and open a PR under your own
feature or main branch. I kindly ask you include a battery of unit tests with your PR to help protect the project
against regressions. Any contributions are very appreciated.
