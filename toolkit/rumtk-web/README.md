# Rumtk-Web

[![Build Status](https://github.com/kiseitai3/rumtk/actions/workflows/check.yml/badge.svg)](https://github.com/kiseitai3/rumtk/actions/workflows/check.yml) [![Crates.io](https://img.shields.io/crates/l/rumtk-web)](LICENSE-LGPL) [![Crates.io](https://img.shields.io/crates/v/rumtk-web)](https://crates.io/crates/rumtk-web) [![Released API docs](https://docs.rs/rumtk-web/badge.svg)](https://docs.rs/rumtk-web) [![Maintenance](https://img.shields.io/maintenance/yes/2026)](https://github.com/kiseitai3/rumtk)

Rust's Universal Medical Toolkit is a toolkit being developed to put together a set of tools and libraries to facilitate
communication and automation in medicine.

```rumtk-web``` is the subframework meant to facilitate the construction of dashboards in Healthcare.

# Goal

+ To provide a very fast and efficient web framework for building reliable dashboards.
+ To provide Server Side Rendering capabilities in healthcare dashboards.
+ To provide a simple and customizable system for putting together these dashboards.

# Features

- [ ] Healthcare Web Dashboard Library
    - [x] Server Side Rendering.
    - [x] High Performance Web framework (see [MedicalMasses L.L.C.'s](https://www.medicalmasses.com/) website which
      uses this framework).
    - [x] Simple old-school organization of web project via the Askama templating engine.
    - [x] Mechanism for component definition.
    - [x] Mechanism for rendering components that is familiar to React users while being based on pure SSR.
    - [x] Mechanism for registering user made components (can be used to override library components).
    - [x] Mechanism for statically defining web page layout in Rust.
    - [x] Mechanism for rendering pages.
    - [x] Mechanism for bundling CSS via component level and as part of a global minified bundle generated on first
      run.
    - [x] Usage of HTMX as a very light JS front end to extend HTML tag functionality.
    - [x] Mechanism for falling back to local copies of HTMX and FontAwesome as needed if CDNs are down.
    - [x] Features exposed via Rust macros.
    - [ ] Tests
    - [ ] Fuzz Targets

# Contributing

In its initial stages, I will be pushing code directly to the main branch. Once basic functionality has been stablished,
everyone including myself is required to open an issue for discussions, fork the project, and open a PR under your own
feature or main branch. I kindly ask you include a battery of unit tests with your PR to help protect the project
against regressions. Any contributions are very appreciated.
