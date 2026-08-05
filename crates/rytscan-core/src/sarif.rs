//! SARIF (Static Analysis Results Interchange Format) v2.1.0 output.
//!
//! Lets RytScan integrate with GitHub Code Scanning and other SARIF-aware
//! code review tooling. See <https://sarifweb.azurewebsites.net/> for the
//! schema.

use crate::report::{Report, Severity};
use crate::rules::all_rules;

const SARIF_SCHEMA: &str = "https://json.schemastore.org/sarif-2.1.0.json";
const INFORMATION_URI: &str = "https://github.com/BreachDirect/RytScan";

fn sarif_level(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical | Severity::High => "error",
        Severity::Medium => "warning",
        Severity::Low | Severity::Info => "note",
    }
}

/// Renders a [`Report`] as a SARIF 2.1.0 run object.
pub fn to_sarif(report: &Report) -> serde_json::Value {
    let rules: Vec<serde_json::Value> = all_rules()
        .iter()
        .map(|rule| {
            serde_json::json!({
                "id": rule.id(),
                "shortDescription": { "text": rule.title() },
                "helpUri": format!("{INFORMATION_URI}/blob/main/docs/rules.md#{}", rule.id().to_lowercase()),
            })
        })
        .collect();

    let results: Vec<serde_json::Value> = report
        .findings
        .iter()
        .map(|finding| {
            serde_json::json!({
                "ruleId": finding.rule_id,
                "level": sarif_level(finding.severity),
                "message": { "text": finding.message },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": { "uri": finding.file },
                        "region": {
                            "startLine": finding.line,
                            "snippet": { "text": finding.snippet },
                        },
                    }
                }],
                "properties": {
                    "recommendation": finding.recommendation,
                    "title": finding.title,
                },
            })
        })
        .collect();

    serde_json::json!({
        "version": "2.1.0",
        "$schema": SARIF_SCHEMA,
        "runs": [{
            "tool": {
                "driver": {
                    "name": "RytScan",
                    "version": report.version,
                    "informationUri": INFORMATION_URI,
                    "rules": rules,
                }
            },
            "results": results,
        }],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::Finding;

    fn sample_report() -> Report {
        let mut report = Report::new("contracts/token", "0.2.0-test");
        report.findings.push(Finding {
            rule_id: "ARITH-001".into(),
            title: "Unchecked arithmetic that can overflow".into(),
            severity: Severity::High,
            message: "Uses `unchecked_add`, which silently wraps on overflow".into(),
            file: "contracts/token/src/lib.rs".into(),
            line: 42,
            snippet: "let total = supply.unchecked_add(amount);".into(),
            recommendation: "Use checked_add().".into(),
        });
        report.finalize(1, 9);
        report
    }

    #[test]
    fn sarif_has_expected_shape() {
        let sarif = to_sarif(&sample_report());
        assert_eq!(sarif["version"], "2.1.0");
        assert_eq!(sarif["runs"][0]["tool"]["driver"]["name"], "RytScan");
        assert_eq!(sarif["runs"][0]["results"][0]["ruleId"], "ARITH-001");
        assert_eq!(sarif["runs"][0]["results"][0]["level"], "error");
        assert_eq!(
            sarif["runs"][0]["results"][0]["locations"][0]["physicalLocation"]["region"]
                ["startLine"],
            42
        );
    }

    #[test]
    fn sarif_levels_follow_github_code_scanning_conventions() {
        assert_eq!(sarif_level(Severity::High), "error");
        assert_eq!(sarif_level(Severity::Medium), "warning");
        assert_eq!(sarif_level(Severity::Low), "note");
    }
}
