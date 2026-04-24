# Documentation Map

This directory contains the main project documentation for `rustos`.

The goal of this documentation set is to help you answer three questions quickly:

1. What is this project?
2. What works today?
3. Where should I read next?

The docs are intentionally small and focused. Each file should have one clear job.

## Start Here

If you are new to the project, read in this order:

1. `../README.md`
2. `architecture.md`
3. `roadmap.md`
4. `testing.md`
5. `unix-like.md`

Then read subsystem notes only when you need them.

## Document Guide

### Project overview

#### `../README.md`
Use this first.

It explains:
- what `rustos` is
- who it is for
- the current project status
- the shortest useful local workflow
- where to go next

### Core project docs

#### `architecture.md`
Read this when you want the stable architectural view.

It explains:
- project principles
- target and boot strategy at a high level
- repository structure
- kernel design direction
- tooling and documentation strategy

This file should stay relatively stable.

#### `roadmap.md`
Read this when you want implementation status and milestone order.

It explains:
- MVP scope
- non-goals
- milestone sequence
- current progress
- release direction

This is the main status document.

#### `testing.md`
Read this when you want to understand validation and contributor workflow.

It explains:
- testing philosophy
- test layers
- when to add unit tests
- when to use QEMU tests
- standard validation commands
- CI testing direction

### Direction docs

#### `unix-like.md`
Read this when you want the top-level Unix-like direction.

It explains:
- what "Unix-like" means in `rustos`
- kernel and user boundary direction
- syscall, task, descriptor, and VFS direction
- what is intentionally deferred

This is the umbrella design note for the Unix-like MVP track.

#### `paging.md`
Read this when you want the paging direction.

It explains:
- why paging matters
- the minimal paging model
- what is deferred
- why heap support remains deferred

#### `syscalls.md`
Read this when you want the syscall direction.

It explains:
- the minimal syscall model
- syscall numbers, errors, and results
- the tiny dispatch sketch
- what is deferred before real ABI work

#### `tasks.md`
Read this when you want the task direction.

It explains:
- the minimal task model
- task identity and state
- the relationship to scheduling
- what is deferred before real multitasking

#### `descriptors.md`
Read this when you want the descriptor direction.

It explains:
- the minimal handle model
- ownership direction
- the tiny task-to-descriptor ownership sketch
- what is deferred before real descriptor tables

### Reference and policy docs

#### `blog-os-adoption.md`
Read this when you want to understand how `rustos` uses `blog_os` as a reference.

It explains:
- what to adopt
- what to adapt
- what to defer
- how external reference material should influence the project

### Decision records

#### `decisions/`
Read these when you want durable decision history.

Decision records explain:
- what decision was made
- why it was made
- what alternatives were rejected
- when the decision should be revisited

## Which Document Owns What?

To keep the docs consistent, each file should own a specific kind of information.

### `README.md`
Owns:
- project overview
- quick start
- contributor entry points

Should not own:
- detailed milestone status
- deep subsystem design notes
- full testing philosophy

### `architecture.md`
Owns:
- stable principles
- system shape
- repository and module boundaries

Should not own:
- detailed milestone progress
- repeated command lists
- subsystem-specific rollout status

### `roadmap.md`
Owns:
- milestone status
- sequencing
- release shape
- success metrics

Should not own:
- repeated architecture rationale
- repeated testing details
- repeated subsystem design prose

### Subsystem docs
Own:
- subsystem-specific design direction
- minimal models
- deferred items
- relationships to other subsystems

Should not own:
- project-wide status reporting
- repeated project philosophy
- repeated contributor workflow details

## Reading by Goal

### I want to run the project
Read:
1. `../README.md`
2. `testing.md`

### I want to understand the architecture
Read:
1. `architecture.md`
2. `roadmap.md`

### I want to understand the Unix-like direction
Read:
1. `unix-like.md`
2. `syscalls.md`
3. `tasks.md`
4. `descriptors.md`

### I want to understand memory direction
Read:
1. `paging.md`
2. `architecture.md`
3. `roadmap.md`

### I want to contribute
Read:
1. `../README.md`
2. `../CONTRIBUTING.md`
3. `testing.md`
4. the subsystem note related to your change

## Documentation Principles

When updating docs in `rustos`, prefer:

- one clear purpose per file
- short and direct wording
- stable concepts in stable docs
- status updates in `roadmap.md`
- subsystem details in subsystem notes
- links instead of repeated explanations

If a sentence is repeated across multiple docs, it is usually a sign that one file should own it and the others should link to it instead.

## Notes

This documentation set is intentionally educational.

That means the docs should help a new contributor:
- understand the project quickly
- find the right level of detail
- avoid confusing implemented behavior with future direction

When in doubt, choose:
- clarity over completeness
- explicit scope over vague ambition
- one source of truth over repeated wording
