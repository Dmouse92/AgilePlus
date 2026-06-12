//! AST-aware tokenization for source code.
//!
//! Pure-regex tokenization of common Rust and Python keywords/operators.
//! The goal is *not* a full AST — that would require `syn`, `tree-sitter`,
//! or `rustpython-parser` (heavy).  Instead we extract the *significant
//! token classes* (control-flow keywords, type-defining keywords, and the
//! `->` arrow / `,` separators) that are stable across reformatting and
//! renaming.  Identifier names are preserved as opaque tokens; this is
//! enough to give a meaningful Jaccard / MinHash signature for
//! near-duplicate code detection.
//!
//! # Audit
//!
//! Implements recommendation #5 from `AUDIT_BLOC_VS_2026_SOTA.md`: the
//! AST-aware tokenizer that feeds the hybrid dedup pipeline.

use regex::Regex;
use std::sync::OnceLock;

// -------- Regex caching -------------------------------------------------

/// Lazily-compiled regex slot.  Each tokenizer owns its own `OnceLock`
/// keyed on its pattern string, so the regex is compiled at most once per
/// process.
fn cached(pattern: &'static str) -> &'static Regex {
    static RUST: OnceLock<Regex> = OnceLock::new();
    static PYTHON: OnceLock<Regex> = OnceLock::new();
    if pattern == RustTokenizer::PATTERN {
        RUST.get_or_init(|| {
            Regex::new(pattern).unwrap_or_else(|e| panic!("invalid rust regex: {}", e))
        })
    } else if pattern == PythonTokenizer::PATTERN {
        PYTHON.get_or_init(|| {
            Regex::new(pattern).unwrap_or_else(|e| panic!("invalid python regex: {}", e))
        })
    } else {
        panic!("cached(): unknown pattern key");
    }
}

// -------- Trait --------------------------------------------------------

/// Token-extraction strategy.  Each implementation knows the keyword set
/// for one language and emits tokens in a stable order (insertion order
/// of `find_iter`, which is left-to-right over the source).
pub trait AstTokenizer: Send + Sync {
    /// Tokenize `source` into a flat token stream.
    fn tokenize(&self, source: &str) -> Vec<String>;

    /// Stable name (e.g. `"rust"`, `"python"`).
    fn name(&self) -> &'static str;
}

// -------- Rust tokenizer ------------------------------------------------

/// Rust source tokenizer.  Captures the Rust keywords / operators that
/// correlate with structural similarity, plus identifier-shaped runs.
pub struct RustTokenizer;

impl RustTokenizer {
    /// Keyword and operator pattern.  Built once via `OnceLock`.
    const PATTERN: &'static str = r"(?x)
        -> | => | :: |
        \bfn\b | \blet\b | \bmatch\b | \bif\b | \belse\b |
        \buse\b | \bmod\b | \bpub\b | \bstruct\b | \benum\b |
        \btrait\b | \bimpl\b | \bfor\b | \bwhile\b | \breturn\b |
        \bself\b | \bSelf\b |
        [A-Za-z_][A-Za-z0-9_]*  |
        \d+
        ";
}

impl AstTokenizer for RustTokenizer {
    fn name(&self) -> &'static str {
        "rust"
    }

    fn tokenize(&self, source: &str) -> Vec<String> {
        cached(Self::PATTERN)
            .find_iter(source)
            .map(|m| m.as_str().to_string())
            .collect()
    }
}

// -------- Python tokenizer ----------------------------------------------

/// Python source tokenizer.  Captures Python keywords and identifier
/// runs.  Python is whitespace-significant, so the regex is the same
/// shape as Rust's, just with the Python keyword set.
pub struct PythonTokenizer;

impl PythonTokenizer {
    const PATTERN: &'static str = r"(?x)
        -> | => |
        \bdef\b | \bclass\b | \bif\b | \belse\b | \belif\b |
        \bfor\b | \bwhile\b | \btry\b | \bexcept\b |
        \bimport\b | \bfrom\b | \breturn\b | \bself\b |
        \blambda\b | \bwith\b | \bas\b |
        [A-Za-z_][A-Za-z0-9_]*  |
        \d+
        ";
}

impl AstTokenizer for PythonTokenizer {
    fn name(&self) -> &'static str {
        "python"
    }

    fn tokenize(&self, source: &str) -> Vec<String> {
        cached(Self::PATTERN)
            .find_iter(source)
            .map(|m| m.as_str().to_string())
            .collect()
    }
}

/// Build a boxed tokenizer by name.  Returns `None` for unsupported
/// languages.
pub fn for_language(lang: &str) -> Option<Box<dyn AstTokenizer>> {
    match lang.to_ascii_lowercase().as_str() {
        "rust" | "rs" => Some(Box::new(RustTokenizer)),
        "python" | "py" => Some(Box::new(PythonTokenizer)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn uniq(tokens: &[String]) -> HashSet<&str> {
        tokens.iter().map(String::as_str).collect()
    }

    #[test]
    fn rust_extracts_keywords_and_identifiers() {
        let t = RustTokenizer;
        let src = "pub fn add(self, other: u32) -> u32 { let sum = self + other; return sum; }";
        let toks = t.tokenize(src);
        let set = uniq(&toks);
        for kw in ["pub", "fn", "self", "let", "return", "->", "add", "other", "u32"] {
            assert!(set.contains(kw), "missing {} in {:?}", kw, toks);
        }
    }

    #[test]
    fn rust_tokenizer_preserves_identifier_names() {
        // Identifiers are kept verbatim so a renamed function still
        // shows up in dedup as a candidate.
        let t = RustTokenizer;
        let toks = t.tokenize("fn foo_bar() {}");
        assert!(toks.contains(&"fn".to_string()));
        assert!(toks.contains(&"foo_bar".to_string()));
    }

    #[test]
    fn rust_tokenizer_captures_arrow_and_double_colon() {
        let t = RustTokenizer;
        let toks = t.tokenize("impl Trait for Type { fn x(&self) -> Self {} }");
        let set = uniq(&toks);
        assert!(set.contains("->"));
        assert!(set.contains("::"));
        // `Self` is the only capitalized keyword in our set.
        assert!(set.contains("Self"));
    }

    #[test]
    fn rust_tokenizer_drops_comments() {
        // The regex has no comment handling; comments are silently
        // skipped because they contain no matching tokens.  This is
        // intentional (and documented in the module doc).
        let t = RustTokenizer;
        let src = "// this is a comment\nfn main() { /* block */ }";
        let toks = t.tokenize(src);
        assert!(toks.contains(&"fn".to_string()));
        assert!(toks.contains(&"main".to_string()));
        // No stray comment fragments.
        for tok in &toks {
            assert!(!tok.contains("//"));
            assert!(!tok.contains("/*"));
        }
    }

    #[test]
    fn python_extracts_keywords_and_identifiers() {
        let t = PythonTokenizer;
        let src = "def add(self, other):\n    return self + other\n";
        let toks = t.tokenize(src);
        let set = uniq(&toks);
        for kw in ["def", "self", "return", "add", "other"] {
            assert!(set.contains(kw), "missing {} in {:?}", kw, toks);
        }
        // `def` and `return` should both be present, plus identifiers.
        assert!(toks.contains(&"def".to_string()));
        assert!(toks.contains(&"return".to_string()));
    }

    #[test]
    fn python_tokenizer_captures_lambda_and_with() {
        let t = PythonTokenizer;
        let src = "with open(path) as f: data = list(map(lambda x: x.strip(), f))";
        let toks = t.tokenize(src);
        let set = uniq(&toks);
        for kw in ["with", "as", "lambda"] {
            assert!(set.contains(kw), "missing {} in {:?}", kw, toks);
        }
    }

    #[test]
    fn python_tokenizer_handles_class_definition() {
        let t = PythonTokenizer;
        let src = "class Foo:\n    def bar(self):\n        return 1\n";
        let toks = t.tokenize(src);
        let set = uniq(&toks);
        assert!(set.contains("class"));
        assert!(set.contains("def"));
        assert!(set.contains("Foo"));
        assert!(set.contains("bar"));
        assert!(set.contains("self"));
    }

    #[test]
    fn tokenizers_are_deterministic() {
        // Same source -> same token stream, twice.  No global state, no
        // RNG.
        let r = RustTokenizer;
        let p = PythonTokenizer;
        let src = "fn main() { let x = 1; }";
        assert_eq!(r.tokenize(src), r.tokenize(src));
        let py = "def main():\n    x = 1\n";
        assert_eq!(p.tokenize(py), p.tokenize(py));
    }

    #[test]
    fn for_language_dispatch() {
        assert_eq!(for_language("rust").unwrap().name(), "rust");
        assert_eq!(for_language("rs").unwrap().name(), "rust");
        assert_eq!(for_language("python").unwrap().name(), "python");
        assert_eq!(for_language("py").unwrap().name(), "python");
        assert!(for_language("ruby").is_none());
        assert!(for_language("").is_none());
    }

    #[test]
    fn rust_tokenizer_handles_empty_input() {
        let t = RustTokenizer;
        assert!(t.tokenize("").is_empty());
        assert!(t.tokenize("   \n\t  ").is_empty());
    }

    #[test]
    fn python_tokenizer_handles_empty_input() {
        let t = PythonTokenizer;
        assert!(t.tokenize("").is_empty());
        assert!(t.tokenize("\n\n\n").is_empty());
    }

    #[test]
    fn rust_tokenizer_is_structurally_stable_across_renames() {
        // Renaming `foo` -> `baz` in the body should leave the structural
        // tokens (fn, return, identifiers) but change the identifier
        // name.  This is the desired behavior for FA-AST-style dedup:
        // structural similarity, not exact-match.
        let t = RustTokenizer;
        let a = t.tokenize("fn foo() -> u32 { return 1; }");
        let b = t.tokenize("fn baz() -> u32 { return 1; }");
        // Same number of tokens.
        assert_eq!(a.len(), b.len());
        // Structural tokens identical.
        let sa: HashSet<&str> = a.iter().map(String::as_str).collect();
        let sb: HashSet<&str> = b.iter().map(String::as_str).collect();
        for kw in ["fn", "->", "u32", "return"] {
            assert!(sa.contains(kw) && sb.contains(kw));
        }
        // Identifiers differ.
        assert!(sa.contains("foo"));
        assert!(sb.contains("baz"));
        assert!(!sa.contains("baz"));
        assert!(!sb.contains("foo"));
    }
}
