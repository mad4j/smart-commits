pub fn build_prompt(preprocessed_diff: &str, short: bool) -> String {
    let format_instructions = if short {
        r#"Generate ONLY a single-line commit message in Conventional Commits format:
<type>(optional scope): <short summary>

Do not include any bullet points or body. Just the summary line."#
    } else {
        r#"Generate a commit message in Conventional Commits format:

<type>(optional scope): <short summary>

- bullet point describing a key change
- another bullet point if needed

Keep the summary line under 72 characters."#
    };

    format!(
        r#"You are an expert software engineer writing a git commit message.

Analyze the following code diff and generate a clear, concise commit message.

## Conventional Commits Format
{format_instructions}

## Supported types:
- feat: A new feature
- fix: A bug fix
- refactor: Code restructuring without feature/fix
- docs: Documentation changes
- test: Adding or updating tests
- chore: Maintenance tasks, dependency updates
- perf: Performance improvements
- build: Build system changes
- ci: CI/CD configuration changes

## Type detection heuristics:
- If new functions/classes/endpoints are added → feat
- If error handling, edge cases, or incorrect behavior is fixed → fix
- If code is reorganized without changing behavior → refactor
- If only comments/docs/README changed → docs
- If test files are modified → test
- If package.json/Cargo.toml/requirements.txt changed (non-feature) → chore
- If benchmarks or algorithmic improvements → perf
- If build scripts/Makefile/CI config changed → build or ci

## Code Diff:
```
{preprocessed_diff}
```

Respond with ONLY the commit message. No explanations, no markdown code blocks, just the raw commit message text."#,
        format_instructions = format_instructions,
        preprocessed_diff = preprocessed_diff
    )
}
