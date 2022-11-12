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

- ARML-lang: The language specification and syntax.
- [ARML-engine](ARML-engine/ARML-engine_docs.md): The formatting and document generating engine. (Possibly written in Rust)
- ARML-api: A REST or graphql API that interacts with the engine and allows us to consume it as clients. (Written in Python)
- ARML-xplat: A cross platform client with Android, Web, and eventually, iOS support. (Written in Flutter) 
- ARML-reach: An communication management system for Networking and Outreach. (Writte in Python)

**Plugins and Extensions**

- ARML-docker: Docker image to deploy an instance of ARML tools.



