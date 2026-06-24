use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub rule_id: String,
    pub title: String,
    pub severity: Severity,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub snippet: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub files_scanned: usize,
    pub rules_run: usize,
    pub findings: usize,
    pub by_severity: std::collections::BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub tool: String,
    pub version: String,
    pub target: String,
    pub summary: Summary,
    pub findings: Vec<Finding>,
}

impl Report {
    pub fn new(target: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            tool: "RytScan".into(),
            version: version.into(),
            target: target.into(),
            summary: Summary {
                files_scanned: 0,
                rules_run: 0,
                findings: 0,
                by_severity: std::collections::BTreeMap::new(),
            },
            findings: Vec::new(),
        }
    }

    pub fn finalize(&mut self, files_scanned: usize, rules_run: usize) {
        self.summary.files_scanned = files_scanned;
        self.summary.rules_run = rules_run;
        self.summary.findings = self.findings.len();
        self.summary.by_severity.clear();
        for finding in &self.findings {
            *self
                .summary
                .by_severity
                .entry(finding.severity.as_str().to_string())
                .or_default() += 1;
        }
    }

    pub fn highest_severity(&self) -> Option<Severity> {
        self.findings.iter().map(|f| f.severity).max()
    }
}
