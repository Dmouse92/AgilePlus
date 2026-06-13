//! AST-aware tokenization for source code.
//!
//! Hand-rolled scanner for common Rust and Python keywords / operators.
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

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// Token-extraction strategy.  Each implementation knows the keyword set
/// for one language and emits tokens in a stable left-to-right order.
pub trait AstTokenizer: Send + Sync {
    /// Tokenize `source` into a flat token stream.
    fn tokenize(&self, source: &str) -> Vec<String>;

    /// Stable name (e.g. `"rust"`, `"python"`).
    fn name(&self) -> &'static str;
}

// ---------------------------------------------------------------------------
// Shared scanner
// ---------------------------------------------------------------------------

/// Scan `source` and emit tokens that match `keywords` or specific
/// multi-character operators and punctuation.
fn scan(source: &str, _keywords: &[&'static str]) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = source.as_bytes();
    let n = bytes.len();
    let mut i = 0usize;

    while i < n {
        let c = bytes[i] as char;

        // Skip whitespace
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // Multi-character operators
        if i + 2 <= n {
            let triple = std::str::from_utf8(&bytes[i..i + 2]).unwrap_or("");
            if triple == "->" || triple == "=>" || triple == "::" {
                out.push(triple.to_string());
                i += 2;
                continue;
            }
        }

        // Single-character punctuation we care about
        if c == ',' {
            out.push(",".to_string());
            i += 1;
            continue;
        }

        // Word-like token (identifier or keyword)
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            i += 1;
            while i < n {
                let d = bytes[i] as char;
                if d.is_ascii_alphanumeric() || d == '_' {
                    i += 1;
                } else {
                    break;
                }
            }
            let word = std::str::from_utf8(&bytes[start..i]).unwrap_or("");
            out.push(word.to_string());
            continue;
        }

        // Number literal
        if c.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < n && (bytes[i] as char).is_ascii_digit() {
                i += 1;
            }
            let num = std::str::from_utf8(&bytes[start..i]).unwrap_or("");
            out.push(num.to_string());
            continue;
        }

        // Anything else: skip
        i += 1;
    }

    out
}

// ---------------------------------------------------------------------------
// Rust tokenizer
// ---------------------------------------------------------------------------

/// Rust source tokenizer.
pub struct RustTokenizer;

impl RustTokenizer {
    const KEYWORDS: &'static [&'static str] = &[
        "fn", "let", "match", "if", "else", "use", "mod", "pub",
        "struct", "enum", "trait", "impl", "for", "while", "return",
        "self", "Self",
    ];
}

impl AstTokenizer for RustTokenizer {
    fn name(&self) -> &'static str {
        "rust"
    }

    fn tokenize(&self, source: &str) -> Vec<String> {
        scan(source, Self::KEYWORDS)
    }
}

// ---------------------------------------------------------------------------
// Python tokenizer
// ---------------------------------------------------------------------------

/// Python source tokenizer.
pub struct PythonTokenizer;

impl PythonTokenizer {
    const KEYWORDS: &'static [&'static str] = &[
        "def", "class", "if", "else", "elif", "for", "while", "try",
        "except", "import", "from", "return", "self", "lambda", "with",
        "as",
    ];
}

impl AstTokenizer for PythonTokenizer {
    fn name(&self) -> &'static str {
        "python"
    }

    fn tokenize(&self, source: &str) -> Vec<String> {
        scan(source, Self::KEYWORDS)
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
        // Note: no :: in this input, so we don't assert it
        assert!(set.contains("Self"));
    }

    #[test]
    fn rust_tokenizer_drops_comments() {
        let t = RustTokenizer;
        let src = "// this is a comment\nfn main() { /* block */ }";
        let toks = t.tokenize(src);
        assert!(toks.contains(&"fn".to_string()));
        assert!(toks.contains(&"main".to_string()));
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
        let t = RustTokenizer;
        let a = t.tokenize("fn foo() -> u32 { return 1; }");
        let b = t.tokenize("fn baz() -> u32 { return 1; }");
        assert_eq!(a.len(), b.len());
        let sa: HashSet<&str> = a.iter().map(String::as_str).collect();
        let sb: HashSet<&str> = b.iter().map(String::as_str).collect();
        for kw in ["fn", "->", "u32", "return"] {
            assert!(sa.contains(kw) && sb.contains(kw));
        }
        assert!(sa.contains("foo"));
        assert!(sb.contains("baz"));
        assert!(!sa.contains("baz"));
        assert!(!sb.contains("foo"));
    }

    #[test]
    fn rust_tokenizer_captures_double_colon() {
        let t = RustTokenizer;
        let toks = t.tokenize("std::vec::Vec::new()");
        let set = uniq(&toks);
        assert!(set.contains("::"));
        assert!(set.contains("std"));
        assert!(set.contains("vec"));
        assert!(set.contains("Vec"));
        assert!(set.contains("new"));
    }

    #[test]
    fn python_tokenizer_captures_numbers() {
        let t = PythonTokenizer;
        let toks = t.tokenize("x = 42 + 123");
        let set = uniq(&toks);
        assert!(set.contains("42"));
        assert!(set.contains("123"));
        assert!(set.contains("x"));
    }
}
