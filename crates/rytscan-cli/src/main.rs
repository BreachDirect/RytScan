use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use rytscan_core::{Report, Scanner, ScanOptions, Severity};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "rytscan", about = "Soroban smart contract security scanner for Stellar")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan Soroban Rust sources for security issues
    Scan {
        /// Path to a Soroban contract file or directory
        path: PathBuf,

        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,

        /// Only run specific rule IDs (repeatable)
        #[arg(long = "rule")]
        rules: Vec<String>,

        /// Include test-only Rust files
        #[arg(long)]
        include_tests: bool,

        /// Fail with exit code 1 when findings meet or exceed this severity
        #[arg(long, value_enum, default_value_t = SeverityThreshold::High)]
        fail_on: SeverityThreshold,
    },

    /// List built-in security rules
    Rules,
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
enum SeverityThreshold {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl From<SeverityThreshold> for Severity {
    fn from(value: SeverityThreshold) -> Self {
        match value {
            SeverityThreshold::Info => Severity::Info,
            SeverityThreshold::Low => Severity::Low,
            SeverityThreshold::Medium => Severity::Medium,
            SeverityThreshold::High => Severity::High,
            SeverityThreshold::Critical => Severity::Critical,
        }
    }
}

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Rules => {
            print_rules();
            Ok(ExitCode::SUCCESS)
        }
        Commands::Scan {
            path,
            format,
            rules,
            include_tests,
            fail_on,
        } => {
            let target = path.canonicalize().with_context(|| format!("invalid scan path: {}", path.display()))?;
            let scanner = Scanner::new(ScanOptions {
                rule_ids: rules,
                include_tests,
            });
            let report = scanner.scan_path(&target, VERSION);
            render_report(&report, format);
            if should_fail(&report, fail_on.into()) {
                Ok(ExitCode::from(1))
            } else {
                Ok(ExitCode::SUCCESS)
            }
        }
    }
}

fn print_rules() {
    println!("RytScan built-in rules:\n");
    for rule in rytscan_core::rules::all_rules() {
        println!("  {} — {}", rule.id(), rule.title());
    }
}

fn render_report(report: &Report, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(report).expect("serialize report"));
        }
        OutputFormat::Text => render_text(report),
    }
}

fn render_text(report: &Report) {
    println!("RytScan v{}", report.version);
    println!("Target: {}", report.target);
    println!(
        "Scanned {} file(s) with {} rule(s) — {} finding(s)\n",
        report.summary.files_scanned, report.summary.rules_run, report.summary.findings
    );

    if report.findings.is_empty() {
        println!("✓ No issues detected.");
        return;
    }

    for finding in &report.findings {
        println!(
            "[{}] {}:{} — {} ({})",
            finding.severity.as_str().to_uppercase(),
            finding.file,
            finding.line,
            finding.title,
            finding.rule_id
        );
        println!("  {}", finding.message);
        if !finding.snippet.is_empty() {
            println!("  > {}", finding.snippet);
        }
        println!("  fix: {}", finding.recommendation);
        println!();
    }
}

fn should_fail(report: &Report, threshold: Severity) -> bool {
    report
        .findings
        .iter()
        .any(|finding| finding.severity >= threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_renderer_handles_empty_report() {
        let report = Report::new("fixtures", VERSION);
        render_text(&report);
    }
}
