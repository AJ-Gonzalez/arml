# ARML: Automated Resume Markup Language

ARML is a a set of tools and interfaces to automate the process of keeping and distributing an up-to-date CV/resume, with minimal hassle and maximum polish.

## Goals and Ideals

0. Produce a high quality resume/CV with cusomizable and extensible format.

1. Use a single source of truth and versioning to maintain accuracy.

2. Have a painless, quick process from information update to distribution. 

3. Be extensible and open source.

## Project Structure

This project will contain some core components as well as plugins and extensions, the scope of which is yet unknown. 

The extended documentation for each Core Component is linked below.

**Core Components:**

- [ARML-lang](ARML-lang/ARML-lang_specification.md): The language specification and syntax.
- [ARML-engine](ARML-engine/ARML-engine_docs.md): The formatting and document generating engine.
- [ARML-api](ARML-api/ARML-api_docs.md): A REST or graphql API that interacts with the engine and allows us to consume it as clients. (Written in Python)
- [ARML-xplat](ARML-xplat/ARML-xplat_docs.md): A cross platform client with Android, Web, and eventually, iOS support. (Written in Flutter) 
- [ARML-reach](ARML-reach/ARML-reach_docs.md): An communication management system for Networking and Outreach. (Writte in Python)

**Plugins and Extensions**

- ARML-podman: Podman image to deploy an instance of ARML tools.

## Code Standards and Documentation

Each component in this repository will have its own documentation file written in Markdown. All component documentation must be linked here.

Coding standards will be as per the official guidelines per language, e.g. PEP8 for Python. 

Commit messages must always be descriptive and short.
