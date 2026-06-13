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

/// Edge case: an empty payload (zero-byte JSON file) should fail validation
/// rather than panic or be silently accepted.
#[test]
fn validate_empty_payload_fails() {
    let repo = TempDir::new().unwrap();
    fs::create_dir(repo.path().join("traces")).unwrap();
    fs::write(repo.path().join("traces/FR-empty.json"), "").unwrap();

    Command::cargo_bin("agileplus-trace-validator")
        .unwrap()
        .args(["validate", repo.path().to_str().unwrap()])
        .assert()
        .failure();
}

/// Edge case: a JSON document missing a required string field should fail
/// schema validation with a clear error (not crash on deserialization).
#[test]
fn validate_trace_missing_required_field_fails() {
    let repo = TempDir::new().unwrap();
    fs::create_dir(repo.path().join("traces")).unwrap();
    // Valid JSON syntax but missing the `fr_id` field the schema requires.
    fs::write(
        repo.path().join("traces/FR-bad.json"),
        r##"{
  "spec_slug": "eco-024-traceability",
  "spec_anchor": "#fr-bad",
  "docs_pages": [],
  "tests": [],
  "code_modules": [],
  "journeys": []
}"##,
    )
    .unwrap();

    Command::cargo_bin("agileplus-trace-validator")
        .unwrap()
        .args(["validate", repo.path().to_str().unwrap()])
        .assert()
        .failure();
}

/// Edge case: a single trace carrying many links across every category
/// (docs, tests, code, journeys) should validate and report the full
/// reference count in stats output.
#[test]
fn validate_trace_with_many_links_succeeds() {
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join("traces")).unwrap();
    fs::create_dir_all(repo.path().join("docs/operations/journeys")).unwrap();
    fs::create_dir_all(repo.path().join("src")).unwrap();

    for path in [
        "docs/traceability.md",
        "docs/architecture.md",
        "docs/operations/journeys/FR-9.md",
        "src/lib.rs",
        "src/main.rs",
    ] {
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(repo.path().join(parent)).unwrap();
        }
        fs::write(repo.path().join(path), "stub").unwrap();
    }
    fs::write(repo.path().join("FUNCTIONAL_REQUIREMENTS.md"), "- FR-9").unwrap();
    fs::write(
        repo.path().join("traces/FR-9.json"),
        r##"{
  "fr_id": "FR-9",
  "spec_slug": "eco-024-traceability",
  "spec_anchor": "#fr-9",
  "docs_pages": ["docs/traceability.md", "docs/architecture.md"],
  "tests": ["tests/cli.rs::validate_trace_with_many_links_succeeds"],
  "code_modules": ["src/lib.rs", "src/main.rs"],
  "journeys": ["docs/operations/journeys/FR-9.md"]
}"##,
    )
    .unwrap();

    Command::cargo_bin("agileplus-trace-validator")
        .unwrap()
        .args(["stats", repo.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("traces: 1"))
        .stdout(contains("references: 6"));
}

/// Edge case: multiple traces should all be counted in stats output.
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

/// Edge case: a trace file whose payload is *only* whitespace (no JSON tokens
/// at all) must fail validation rather than panic or be silently treated as
/// an empty object. This complements the zero-byte empty-payload test.
#[test]
fn validate_whitespace_only_payload_fails() {
    let repo = TempDir::new().unwrap();
    fs::create_dir(repo.path().join("traces")).unwrap();
    fs::write(
        repo.path().join("traces/FR-whitespace.json"),
        "   \n\t  \n",
    )
    .unwrap();

    Command::cargo_bin("agileplus-trace-validator")
        .unwrap()
        .args(["validate", repo.path().to_str().unwrap()])
        .assert()
        .failure();
}

/// Edge case: a trace file containing a JSON value of the wrong shape
/// (a top-level array instead of the expected object) must fail validation
/// with a clear schema error rather than crashing the validator.
#[test]
fn validate_malformed_json_array_payload_fails() {
    let repo = TempDir::new().unwrap();
    fs::create_dir(repo.path().join("traces")).unwrap();
    // Valid JSON syntax (a top-level array) but the wrong shape for a trace.
    fs::write(
        repo.path().join("traces/FR-array.json"),
        r##"["not", "a", "trace", "object"]"##,
    )
    .unwrap();

    Command::cargo_bin("agileplus-trace-validator")
        .unwrap()
        .args(["validate", repo.path().to_str().unwrap()])
        .assert()
        .failure();
}

/// Edge case: many traces each carrying many links should all be counted,
/// and stats output should reflect the aggregate across every category.
/// This stresses the multi-trace + multi-link code path together.
#[test]
fn validate_many_traces_with_many_links_succeeds() {
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join("traces")).unwrap();
    fs::create_dir_all(repo.path().join("src")).unwrap();
    fs::create_dir_all(repo.path().join("docs")).unwrap();
    fs::create_dir_all(repo.path().join("docs/operations/journeys")).unwrap();

    // Create the target files that will be referenced.
    for path in [
        "docs/trace.md",
        "docs/architecture.md",
        "src/lib.rs",
        "src/main.rs",
    ] {
        fs::write(repo.path().join(path), "stub").unwrap();
    }

    let trace_count = 4;
    let fr_ids: Vec<String> = (1..=trace_count).map(|i| format!("FR-multi-{i}")).collect();
    for fr in &fr_ids {
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
  "spec_slug": "eco-024-traceability",
  "spec_anchor": "#{fr}",
  "docs_pages": ["docs/trace.md", "docs/architecture.md"],
  "tests": ["tests/edge_cases.rs::validate_many_traces_with_many_links_succeeds"],
  "code_modules": ["src/lib.rs", "src/main.rs"],
  "journeys": ["docs/operations/journeys/{fr}.md"]
}}"##
            ),
        )
        .unwrap();
    }

    // FUNCTIONAL_REQUIREMENTS.md must list every FR or the validator will
    // treat the missing traces as errors. The aggregated link count is
    // (2 docs + 1 test + 2 code + 1 journey) * trace_count = 6 * 4 = 24.
    let reqs = fr_ids
        .iter()
        .map(|fr| format!("- {fr}"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(repo.path().join("FUNCTIONAL_REQUIREMENTS.md"), reqs).unwrap();

    Command::cargo_bin("agileplus-trace-validator")
        .unwrap()
        .args(["stats", repo.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains(format!("traces: {trace_count}")))
        .stdout(contains("references: 24"));
}
