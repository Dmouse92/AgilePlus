//! Lockfile parsers for the four supported ecosystems.
//!
//! Each parser turns a lockfile into a `Vec<Dependency>` with concrete
//! (resolved) versions where possible.
//!
//! Implemented formats:
//! - Cargo: `Cargo.lock` (TOML with `[[package]]` entries)
//! - Npm: `package-lock.json` v2/v3 (the `packages` map, with `node_modules/X` keys)
//! - PyPI: `requirements.txt` (PEP 508-ish, very forgiving)
//! - Go: `go.sum` (one `module version h1:hash` per line)

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::dependency::{Dependency, Ecosystem};
use crate::ecosystem;
use crate::error::{Error, Result};

/// Parse a lockfile at `path` and return its resolved dependencies.
pub fn parse_lockfile(path: &Path) -> Result<Vec<Dependency>> {
    let ecosystem = ecosystem::ecosystem_for_lockfile(path).ok_or_else(|| {
        Error::Manifest(format!(
            "unsupported lockfile filename: {}",
            path.display()
        ))
    })?;
    let raw = fs::read_to_string(path)?;
    let lockfile_path = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("lockfile")
        .to_string();
    let deps = match ecosystem {
        Ecosystem::Cargo => parse_cargo_lock(&raw, &lockfile_path)?,
        Ecosystem::Npm => parse_npm_lock(&raw, &lockfile_path)?,
        Ecosystem::Pypi => parse_requirements_txt(&raw, &lockfile_path)?,
        Ecosystem::Go => parse_go_sum(&raw, &lockfile_path)?,
        Ecosystem::Other => Vec::new(),
    };
    Ok(deps)
}

/// Parse the contents of a `Cargo.lock` file.
pub fn parse_cargo_lock(raw: &str, lockfile_path: &str) -> Result<Vec<Dependency>> {
    let lock: CargoLock = toml::from_str(raw)
        .map_err(|e| Error::Manifest(format!("Cargo.lock: {e}")))?;
    let mut out = Vec::new();
    for pkg in lock.package {
        // Skip the root package (no name == root workspace entry).
        if pkg.name.is_empty() {
            continue;
        }
        out.push(
            Dependency::new(pkg.name, pkg.version, Ecosystem::Cargo, lockfile_path)
                .with_section("package"),
        );
    }
    Ok(out)
}

/// Parse the contents of an npm `package-lock.json` v2/v3.
pub fn parse_npm_lock(raw: &str, lockfile_path: &str) -> Result<Vec<Dependency>> {
    let lock: NpmLock = serde_json::from_str(raw)
        .map_err(|e| Error::Manifest(format!("package-lock.json: {e}")))?;
    let mut out = Vec::new();
    for (key, entry) in lock.packages {
        // The `""` key is the root package; skip it.
        if key.is_empty() {
            continue;
        }
        // The key is typically `node_modules/<name>`; strip the prefix.
        let name = key
            .strip_prefix("node_modules/")
            .unwrap_or(&key)
            .to_string();
        let version = entry.version.unwrap_or_else(|| "*".into());
        out.push(
            Dependency::new(name, version, Ecosystem::Npm, lockfile_path)
                .with_section("packages"),
        );
    }
    Ok(out)
}

/// Parse the contents of a `requirements.txt` file.
///
/// Permissive: handles bare names, `name==1.2.3`, `name>=1.0`, hashes,
/// `-r other.txt` (recursively followed if `base_dir` is provided to
/// the file-level entry point), and `# comment` lines. This is best-effort.
pub fn parse_requirements_txt(raw: &str, lockfile_path: &str) -> Result<Vec<Dependency>> {
    let mut out = Vec::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("-r ") {
            // -r other.txt: we don't recurse here (caller's job), but we
            // emit a placeholder so the dependency is at least visible.
            let target = rest.trim().to_string();
            out.push(
                Dependency::new(target, "*", Ecosystem::Pypi, lockfile_path)
                    .with_section("-r"),
            );
            continue;
        }
        let (name, version) = split_requirements_line(line);
        if name.is_empty() || name == "*" {
            continue;
        }
        out.push(
            Dependency::new(name, version, Ecosystem::Pypi, lockfile_path)
                .with_section("requirements"),
        );
    }
    Ok(out)
}

/// Parse the contents of a `go.sum` file.
pub fn parse_go_sum(raw: &str, lockfile_path: &str) -> Result<Vec<Dependency>> {
    // Dedupe: each (module, version) appears twice in go.sum
    // (once for `h1:...` and once for `/go.mod h1:...`).
    let mut seen: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut out = Vec::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();
        if name.is_empty() || version.is_empty() {
            continue;
        }
        // Skip the `/go.mod` h1 hash lines (they only carry the module
        // path, not a real dependency).
        if line.contains("/go.mod h1:") {
            continue;
        }
        if seen.insert((name.clone(), version.clone())) {
            out.push(
                Dependency::new(name, version, Ecosystem::Go, lockfile_path)
                    .with_section("go.sum"),
            );
        }
    }
    Ok(out)
}

// ---- internal: serde shapes ----------------------------------------------

#[derive(Debug, Default, Deserialize)]
struct CargoLock {
    #[serde(default)]
    package: Vec<CargoLockPackage>,
}

#[derive(Debug, Default, Deserialize)]
struct CargoLockPackage {
    #[serde(default)]
    name: String,
    #[serde(default)]
    version: String,
}

#[derive(Debug, Default, Deserialize)]
struct NpmLock {
    /// Keyed by `node_modules/<name>` (or `""` for the root).
    #[serde(default)]
    packages: BTreeMap<String, NpmLockEntry>,
}

#[derive(Debug, Default, Deserialize)]
struct NpmLockEntry {
    #[serde(default)]
    version: Option<String>,
}

// ---- internal: tiny helpers ---------------------------------------------

/// Split a `requirements.txt` line into `(name, version)`. Handles
/// `name`, `name==1.2.3`, `name>=1.0`, `name~=1.0`, `@./path`, hashes.
fn split_requirements_line(line: &str) -> (String, String) {
    // Strip trailing comment.
    let line = line.split('#').next().unwrap_or("").trim();
    // Skip global options.
    if line.starts_with('-') && !line.starts_with("--hash=") {
        return ("*".into(), "*".into());
    }
    // Find the first operator: ==, >=, <=, ~=, !=, ===
    let bytes = line.as_bytes();
    let mut split_at = line.len();
    for (i, _) in bytes.iter().enumerate() {
        if i + 1 < bytes.len() && bytes[i] == b'=' && bytes[i + 1] == b'=' {
            split_at = i;
            break;
        }
        if i + 1 < bytes.len() && bytes[i] == b'>' && bytes[i + 1] == b'=' {
            split_at = i;
            break;
        }
        if i + 1 < bytes.len() && bytes[i] == b'<' && bytes[i + 1] == b'=' {
            split_at = i;
            break;
        }
        if bytes[i] == b'~' || bytes[i] == b'!' {
            split_at = i;
            break;
        }
    }
    let (name, version) = line.split_at(split_at);
    let name = name.trim().to_string();
    let version = version.trim_start_matches('=').trim().to_string();
    if name.is_empty() {
        ("*".into(), "*".into())
    } else {
        (name, version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CARGO_LOCK: &str = r#"
version = 3

[[package]]
name = "demo"
version = "0.0.1"

[[package]]
name = "serde"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"

[[package]]
name = "tokio"
version = "1.46.0"
"#;

    const NPM_LOCK: &str = r#"
{
  "name": "demo",
  "version": "0.0.1",
  "lockfileVersion": 3,
  "packages": {
    "": { "name": "demo", "version": "0.0.1" },
    "node_modules/lodash": { "version": "4.17.21" },
    "node_modules/react": { "version": "18.2.0" }
  }
}
"#;

    const REQUIREMENTS: &str = r#"
# comment
requests==2.31.0
flask>=2.0
click
django ~= 4.2
-r other-requirements.txt
"#;

    const GO_SUM: &str = r#"
github.com/foo/bar v1.2.3 h1:abc=
github.com/foo/bar v1.2.3/go.mod h1:def=
github.com/baz/qux v0.4.0 h1:xyz=
github.com/baz/qux v0.4.0/go.mod h1:uvw=
"#;

    #[test]
    fn cargo_lock_parses_packages() {
        let deps = parse_cargo_lock(CARGO_LOCK, "Cargo.lock").unwrap();
        assert_eq!(deps.len(), 3);
        let serde = deps.iter().find(|d| d.name == "serde").unwrap();
        assert_eq!(serde.version, "1.0.0");
    }

    #[test]
    fn npm_lock_parses_packages() {
        let deps = parse_npm_lock(NPM_LOCK, "package-lock.json").unwrap();
        // Root "" entry is skipped.
        assert_eq!(deps.len(), 2);
        let lodash = deps.iter().find(|d| d.name == "lodash").unwrap();
        assert_eq!(lodash.version, "4.17.21");
        let react = deps.iter().find(|d| d.name == "react").unwrap();
        assert_eq!(react.version, "18.2.0");
    }

    #[test]
    fn requirements_parses_pep508ish() {
        let deps = parse_requirements_txt(REQUIREMENTS, "requirements.txt").unwrap();
        let names: Vec<&str> = deps.iter().map(|d| d.name.as_str()).collect();
        assert!(names.contains(&"requests"));
        assert!(names.contains(&"flask"));
        assert!(names.contains(&"click"));
        assert!(names.contains(&"django"));
        let req = deps.iter().find(|d| d.name == "requests").unwrap();
        assert_eq!(req.version, "2.31.0");
        let flask = deps.iter().find(|d| d.name == "flask").unwrap();
        assert_eq!(flask.version, "2.0");
    }

    #[test]
    fn go_sum_dedupes_module_version_pairs() {
        let deps = parse_go_sum(GO_SUM, "go.sum").unwrap();
        // Both modules have h1 + /go.mod lines; we dedupe to one entry per pair.
        assert_eq!(deps.len(), 2);
        let bar = deps.iter().find(|d| d.name == "github.com/foo/bar").unwrap();
        assert_eq!(bar.version, "v1.2.3");
    }

    #[test]
    fn unsupported_lockfile_filename_errors() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("poetry.lock");
        std::fs::write(&p, "# poetry lockfile v1\n").unwrap();
        let err = parse_lockfile(&p).unwrap_err();
        assert!(err.to_string().contains("unsupported lockfile"));
    }

    #[test]
    fn split_requirements_line_handles_operators() {
        assert_eq!(split_requirements_line("a==1.0"), ("a".into(), "1.0".into()));
        assert_eq!(split_requirements_line("a>=1.0"), ("a".into(), "1.0".into()));
        assert_eq!(split_requirements_line("a<=1.0"), ("a".into(), "1.0".into()));
        assert_eq!(split_requirements_line("a~=1.0"), ("a".into(), "1.0".into()));
        assert_eq!(split_requirements_line("a"), ("a".into(), "".into()));
    }
}
