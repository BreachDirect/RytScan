use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::report::Report;
use crate::rules::{rules_by_ids, RuleContext};

#[derive(Debug, Clone, Default)]
pub struct ScanOptions {
    pub rule_ids: Vec<String>,
    pub include_tests: bool,
}

pub struct Scanner {
    options: ScanOptions,
}

impl Scanner {
    pub fn new(options: ScanOptions) -> Self {
        Self { options }
    }

    pub fn scan_path(&self, target: &Path, version: &str) -> Report {
        let mut report = Report::new(display_path(target), version);
        let rules = rules_by_ids(&self.options.rule_ids);
        let rule_count = rules.len();
        let files = collect_rust_files(target, self.options.include_tests);
        let file_count = files.len();

        for file in &files {
            let source = match std::fs::read_to_string(&file) {
                Ok(content) => content,
                Err(_) => continue,
            };
            let lines: Vec<String> = source.lines().map(|line| line.to_string()).collect();
            let rel = file
                .strip_prefix(target)
                .unwrap_or(&file)
                .to_string_lossy()
                .to_string();
            let ctx = RuleContext {
                file: &rel,
                source: &source,
                lines: &lines,
            };

            for rule in &rules {
                report.findings.extend(rule.run(&ctx));
            }
        }

        report.finalize(file_count, rule_count);
        report
            .findings
            .sort_by(|a, b| (a.file.clone(), a.line, a.rule_id.clone()).cmp(&(b.file.clone(), b.line, b.rule_id.clone())));
        report
    }
}

fn collect_rust_files(target: &Path, include_tests: bool) -> Vec<PathBuf> {
    if target.is_file() {
        return if is_rust_file(target) {
            vec![target.to_path_buf()]
        } else {
            Vec::new()
        };
    }

    WalkDir::new(target)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| is_rust_file(path) && (include_tests || !is_test_only_path(path)))
        .collect()
}

fn is_rust_file(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "rs")
}

fn is_test_only_path(path: &Path) -> bool {
    let path = path.to_string_lossy();
    path.contains("/tests/") || path.ends_with("_test.rs") || path.contains("/test/")
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_fixture_directory() {
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vulnerable-vault/src");
        let scanner = Scanner::new(ScanOptions::default());
        let report = scanner.scan_path(&fixture, "0.1.0-test");
        assert!(report.summary.findings > 0);
    }
}
