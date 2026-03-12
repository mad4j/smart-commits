# Advanced Repository Generation Prompt

# Project: smart-commit

## Task

You are an expert Rust developer and software architect.

Your task is to generate a **complete Rust CLI project** called:

`smart-commit`

The project must compile successfully and implement a tool that
generates commit messages using a **local Large Language Model (LLM)**.

The tool must support both **Git** and **SVN (Subversion)** repositories
and must be designed with clean architecture, modular modules, and
idiomatic Rust.

Your output must include the **entire repository structure and all
source files**.

The produced code must be ready to compile using:

    cargo build

------------------------------------------------------------------------

# Output Rules

Follow these rules strictly:

1.  Output the **repository tree first**
2.  Then output **every file**
3.  Each file must appear in the following format:

FILE: path/to/file.rs

``` rust
<code>
```

4.  Never produce pseudocode.
5.  Write complete compilable Rust code.
6.  Include comments explaining non‑trivial logic.

------------------------------------------------------------------------

# Project Goal

`smart-commit` is a developer tool that analyzes repository changes and
generates a structured commit message using a **local LLM server**.

The system must:

-   work fully **offline**
-   support **Git and SVN**
-   generate **Conventional Commit messages**
-   detect commit types automatically
-   optionally perform the commit automatically
-   support **LLM streaming responses**

------------------------------------------------------------------------

# Required Features

## 1 Repository Detection

Automatically detect the repository type.

Detection rules:

If `.git` exists → use Git\
If `.svn` exists → use SVN

Allow override:

    smart-commit --vcs git
    smart-commit --vcs svn

------------------------------------------------------------------------

# 2 Diff Extraction

The tool must extract repository changes.

### Git

    git diff --staged
    git diff HEAD

### SVN

    svn diff
    svn status

The extracted diff must include:

-   filenames
-   added lines
-   removed lines

Remove irrelevant metadata.

------------------------------------------------------------------------

# 3 Diff Preprocessing

Large diffs must be reduced before sending them to the LLM.

The preprocessing stage must:

-   remove metadata lines
-   preserve filenames
-   keep added and removed lines
-   limit the total number of lines
-   group changes by file

Example cleaned diff:

    File: src/parser.rs

    + fn parse_template_args() {
    +     if args.is_empty() {
    +         return Err(ParseError::EmptyTemplate);
    +     }
    + }

------------------------------------------------------------------------

# 4 Commit Message Format

Use **Conventional Commits**.

Format:

    <type>(scope): short summary

Followed by bullet points.

Example:

    feat(parser): add validation for empty template arguments

    - detect empty template parameter list
    - introduce ParseError::EmptyTemplate
    - prevent parser crash

------------------------------------------------------------------------

# 5 Commit Type Detection

The tool must infer the commit type automatically.

Supported types:

-   feat
-   fix
-   refactor
-   docs
-   test
-   chore
-   perf
-   build
-   ci

Detection heuristics:

feat\
New functionality such as new functions, modules, APIs.

fix\
Bug fixes, added checks, error handling.

refactor\
Structural code changes without behavior change.

docs\
Documentation-only changes.

test\
Changes inside test files.

chore\
Maintenance tasks.

The model should infer type using:

-   filenames
-   diff semantics
-   change patterns.

------------------------------------------------------------------------

# 6 LLM Integration

The tool must call a **local LLM server over HTTP**.

Example endpoint:

    POST /api/generate

Example body:

``` json
{
  "model": "deepseek-coder",
  "prompt": "...",
  "stream": false
}
```

The endpoint must be configurable.

------------------------------------------------------------------------

# 7 Streaming LLM Support (Important Feature)

The tool must support **streaming responses from the LLM**.

When streaming is enabled:

-   The tool reads tokens incrementally
-   Displays partial responses in real time
-   Assembles the final commit message

Example streaming request:

``` json
{
  "model": "deepseek-coder",
  "prompt": "...",
  "stream": true
}
```

Implementation requirements:

-   support chunked HTTP responses
-   process tokens incrementally
-   display partial output to terminal
-   collect final message buffer

The streaming client should:

-   use async Rust
-   handle partial JSON chunks
-   support cancellation.

------------------------------------------------------------------------

# 8 CLI Interface

Example commands:

    smart-commit
    smart-commit --dry-run
    smart-commit --apply
    smart-commit --vcs svn
    smart-commit --short
    smart-commit --stream

Flags:

`--dry-run` → preview commit message\
`--apply` → perform commit automatically\
`--stream` → enable streaming output\
`--short` → only generate summary line

------------------------------------------------------------------------

# 9 Commit Execution

Git:

    git commit -m "<message>"

SVN:

    svn commit -m "<message>"

------------------------------------------------------------------------

# 10 Config File

Support configuration file:

    ~/.smart-commit/config.toml

Example:

``` toml
model = "deepseek-coder"
endpoint = "http://localhost:11434"
max_diff_lines = 400
stream = true
```

------------------------------------------------------------------------

# 11 Required Rust Dependencies

Use these crates:

    clap
    serde
    serde_json
    reqwest
    tokio
    anyhow
    futures

Streaming may use:

-   async streams
-   reqwest streaming body
-   tokio runtime.

------------------------------------------------------------------------

# 12 Project Structure

    smart-commit/
     ├── Cargo.toml
     └── src/
         ├── main.rs
         ├── cli.rs
         ├── config.rs
         ├── llm/
         │     ├── client.rs
         │     └── stream.rs
         ├── vcs/
         │     ├── git.rs
         │     └── svn.rs
         ├── diff/
         │     └── preprocess.rs
         ├── prompt/
         │     └── builder.rs
         └── output/
               └── formatter.rs

------------------------------------------------------------------------

# 13 Optional Advanced Features

Nice-to-have features:

-   semantic scope detection
-   PR description generation
-   release notes generation
-   Git hook integration
-   AST-aware analysis using tree-sitter

------------------------------------------------------------------------

# Quality Requirements

The generated project must:

-   compile successfully
-   include error handling
-   follow idiomatic Rust patterns
-   clearly separate modules
-   support async execution
-   implement streaming correctly.

------------------------------------------------------------------------

# Final Instruction

Generate the complete repository for **smart-commit** now.
