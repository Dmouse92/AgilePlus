use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use tempfile::TempDir;

/// Edge case: an empty `traces/` directory should validate cleanly with zero traces.
#[test]
fn validate_empty_traces_directory_succeeds() {
    let repo = TempDir::new().unwrap();
    fs::create_dir(repo.path().join("traces")).unwrap();
    fs::write(repo.path().join("FUNCTIONAL_REQUIREMENTS.md"), "").unwrap();

    Command::cargo_bin("agileplus-trace-validator")
        .unwrap()
        .args(["validate", repo.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("validated 0 trace files"));
}

/// Edge case: a malformed JSON trace file should cause the validator to fail
/// rather than panic or silently ignore the file.
#[test]
fn validate_malformed_json_trace_fails() {
    let repo = TempDir::new().unwrap();
    fs::create_dir(repo.path().join("traces")).unwrap();
    // Truncated JSON — not valid syntax and missing required fields.
    fs::write(
        repo.path().join("traces/FR-broken.json"),
        r##"{ "fr_id": "FR-broken", "spec_slug": "##,
    )
    .unwrap();

    Command::cargo_bin("agileplus-trace-validator")
        .unwrap()
        .args(["validate", repo.path().to_str().unwrap()])
        .assert()
        .failure();
}

/// Edge case: multiple valid trace files should all be counted in stats output.
#[test]
fn stats_reports_multiple_traces() {
    let repo = TempDir::new().unwrap();
    fs::create_dir(repo.path().join("traces")).unwrap();
    fs::create_dir_all(repo.path().join("src")).unwrap();
    fs::create_dir_all(repo.path().join("docs/operations/journeys")).unwrap();
    fs::write(repo.path().join("docs/traceability.md"), "trace docs").unwrap();

    for (i, fr) in ["FR-1", "FR-2", "FR-3"].iter().enumerate() {
        fs::write(repo.path().join("src/module.rs"), "fn x() {}").unwrap();
        fs::write(
            repo.path().join(format!("docs/operations/journeys/{fr}.md")),
            "journey",
        )
        .unwrap();
        fs::write(
            repo.path().join(format!("traces/{fr}.json")),
            format!(
                r##"{{
  "fr_id": "{fr}",
  "spec_slug": "eco-{i:03}-traceability",
  "spec_anchor": "#{fr}",
  "docs_pages": ["docs/traceability.md"],
  "tests": ["tests/cli.rs::stats_reports_multiple_traces"],
  "code_modules": ["src/module.rs"],
  "journeys": ["docs/operations/journeys/{fr}.md"]
}}"##
            ),
        )
        .unwrap();
    }
    fs::write(
        repo.path().join("FUNCTIONAL_REQUIREMENTS.md"),
        "- FR-1\n- FR-2\n- FR-3",
    )
    .unwrap();

    Command::cargo_bin("agileplus-trace-validator")
        .unwrap()
        .args(["stats", repo.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("traces: 3"));
}
