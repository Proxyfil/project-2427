# Polytech DevOps - System Programming Project

## Rules

Students MUST pick one out of the [four proposed topics](topics).

Before the project deadline, students MUST turn in an [architecture document](#architecture-and-documentation) and an [implementation](#implementation) for the selected topic, in the form of a [GitHub pull request](#project-contribution).

## Topics

Four different topics are proposed:

1. [A networking port scanner](topics/port-scanner).
1. [A peer-to-peer file transfer protocol](topics/p2p-transfer-protocol).
1. [A web scraper](topics/web-scraper).
1. [A tic-tac-toe AI agent](topics/tic-tac-toe).

## Grade

Your work will be evaluated based on the following criteria:

### Architecture and Documentation

**This accounts for 40% of the final grade**

You MUST provide a short, `markdown` formatted and English written document describing your project architecture.

It MUST live under a projects top level folder called `docs/`, e.g. `docs/architecture.md`.

It SHOULD at least contain the following sections:

1. Project definition: What is it? What are the goals of the tool/project?
1. Components and modules: Describe which modules compose your project, and how they interact together. Briefly justify why you architectured it this way.
1. Usage: How can one use it? Give usage examples.

### Implementation

**This accounts for 40% of the final grade**

The project MUST be implemented in Rust.

The implementation MUST be formatted, build without warnings (including `clippy` warnings) and commented.

The implementation modules and crates MAY be unit tested.

### Project Contribution

**This accounts for 20% of the final grade**

The project MUST be submitted as one single GitHub pull request (PR) against the [current](https://github.com/dev-sys-do/project-2427) repository, for the selected project.

For example, a student picking the `p2p-transfer-protocol` topic MUST send a PR that adds all deliverables (source code, documentation) to the `topics/p2p-transfer-protocol/` folder.

All submitted PRs will not be evaluated until the project deadline. They can thus be incomplete, rebased, closed, and modified until the project deadline.

A pull request quality is evaluated on the following criteria:
* Commit messages: Each git commit message should provide a clear description and explanation of what the corresponding change brings and does.
* History: The pull request git history MUST be linear (no merge points) and SHOULD represent the narrative of the underlying work. It is a representation of the author's logical work in putting the implementation together.

A very good reference on the topic: https://github.blog/developer-skills/github/write-better-commits-build-better-projects/

### Grade Factor

All proposed topics have a grade factor, describing their relative complexity.

The final grade is normalized against the selected topic's grade factor: `final_grade = grade * topic_grade_factor`.

For example, a grade of `8/10` for a topic which grade factor is `1.1` will generate a final grade of `8.8/10`.


## Deadline

All submitted PRs will be evaluated on September 24th, 2025 at 11:00 PM UTC.
