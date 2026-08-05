use crate::report::{Finding, Severity};

pub struct RuleContext<'a> {
    pub file: &'a str,
    pub source: &'a str,
    pub lines: &'a [String],
}

pub trait Rule {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding>;
}

pub fn all_rules() -> Vec<Box<dyn Rule + Send + Sync>> {
    vec![
        Box::new(AuthMissingRule),
        Box::new(PanicUsageRule),
        Box::new(MissingEventsRule),
        Box::new(UncheckedTransferRule),
        Box::new(TtlExtensionRule),
        Box::new(UnsafeTemporaryStorageRule),
        Box::new(UncheckedArithmeticRule),
        Box::new(AssertUsageRule),
        Box::new(UnsafeBlockRule),
    ]
}

pub fn rules_by_ids(ids: &[String]) -> Vec<Box<dyn Rule + Send + Sync>> {
    let all = all_rules();
    if ids.is_empty() {
        return all;
    }
    all.into_iter()
        .filter(|rule| ids.iter().any(|id| id == rule.id()))
        .collect()
}

struct AuthMissingRule;

impl Rule for AuthMissingRule {
    fn id(&self) -> &'static str {
        "AUTH-001"
    }

    fn title(&self) -> &'static str {
        "Missing require_auth on privileged function"
    }

    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding> {
        let mut findings = Vec::new();
        for function in extract_functions(ctx.source) {
            if !looks_like_contract_fn(&function.body) {
                continue;
            }
            if !is_state_changing(&function.body) {
                continue;
            }
            if function.body.contains("require_auth") {
                continue;
            }
            if function.name.starts_with("get_") || function.name.starts_with("view_") {
                continue;
            }

            let line = function.start_line;
            findings.push(Finding {
                rule_id: self.id().into(),
                title: self.title().into(),
                severity: Severity::High,
                message: format!(
                    "Function `{}` modifies state but never calls require_auth()",
                    function.name
                ),
                file: ctx.file.to_string(),
                line,
                snippet: snippet_at(ctx.lines, line),
                recommendation: "Call env.storage().instance().get(&DataKey::Admin) or the relevant signer address, then invoke require_auth() before mutating vault state.".into(),
            });
        }
        findings
    }
}

struct PanicUsageRule;

impl Rule for PanicUsageRule {
    fn id(&self) -> &'static str {
        "PANIC-001"
    }

    fn title(&self) -> &'static str {
        "Panic-prone error handling in contract code"
    }

    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding> {
        let patterns = [
            (".unwrap()", "unwrap()"),
            (".expect(", "expect()"),
            ("panic!(", "panic!()"),
        ];
        let mut findings = Vec::new();

        for (line_no, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            for (pattern, label) in patterns {
                if line.contains(pattern) {
                    findings.push(Finding {
                        rule_id: self.id().into(),
                        title: self.title().into(),
                        severity: Severity::Medium,
                        message: format!("Uses {label} which aborts the entire transaction"),
                        file: ctx.file.to_string(),
                        line: line_no + 1,
                        snippet: trimmed.to_string(),
                        recommendation: "Return a typed contract error (Result<T, ContractError>) instead of panicking.".into(),
                    });
                }
            }
        }
        findings
    }
}

struct MissingEventsRule;

impl Rule for MissingEventsRule {
    fn id(&self) -> &'static str {
        "EVENT-001"
    }

    fn title(&self) -> &'static str {
        "State-changing function without Soroban event emission"
    }

    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding> {
        let mut findings = Vec::new();
        for function in extract_functions(ctx.source) {
            if !looks_like_contract_fn(&function.body) {
                continue;
            }
            if !is_state_changing(&function.body) {
                continue;
            }
            if function.body.contains("env.events()")
                || function.body.contains("env.events().publish")
            {
                continue;
            }
            if function.name.starts_with("initialize") || function.name.starts_with("__") {
                continue;
            }

            findings.push(Finding {
                rule_id: self.id().into(),
                title: self.title().into(),
                severity: Severity::Low,
                message: format!(
                    "Function `{}` changes state but does not emit env.events().publish(...)",
                    function.name
                ),
                file: ctx.file.to_string(),
                line: function.start_line,
                snippet: snippet_at(ctx.lines, function.start_line),
                recommendation: "Publish structured events for deposits, withdrawals, and admin actions so indexers can track activity off-chain.".into(),
            });
        }
        findings
    }
}

struct UncheckedTransferRule;

impl Rule for UncheckedTransferRule {
    fn id(&self) -> &'static str {
        "TOKEN-001"
    }

    fn title(&self) -> &'static str {
        "Unchecked SEP-41 token transfer"
    }

    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_no, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            let has_transfer = trimmed.contains(".transfer(") || trimmed.contains("transfer(&env");
            let checks_result = trimmed.contains("if !")
                || trimmed.contains("match ")
                || trimmed.contains("ensure!(")
                || trimmed.contains("?;")
                || trimmed.contains(".is_ok()");
            if has_transfer && !checks_result {
                findings.push(Finding {
                    rule_id: self.id().into(),
                    title: self.title().into(),
                    severity: Severity::High,
                    message: "Token transfer return value is not checked".into(),
                    file: ctx.file.to_string(),
                    line: line_no + 1,
                    snippet: trimmed.to_string(),
                    recommendation: "Check the bool returned by token.transfer() or use a helper that maps failure to ContractError.".into(),
                });
            }
        }
        findings
    }
}

struct TtlExtensionRule;

impl Rule for TtlExtensionRule {
    fn id(&self) -> &'static str {
        "TTL-001"
    }

    fn title(&self) -> &'static str {
        "Persistent storage write without TTL extension"
    }

    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding> {
        let mut findings = Vec::new();
        for function in extract_functions(ctx.source) {
            let uses_persistent = function.body.contains(".persistent().set(")
                || function.body.contains(".persistent().update(");
            let extends_ttl = function.body.contains("extend_ttl")
                || function.body.contains("extend_instance_ttl");
            if uses_persistent && !extends_ttl {
                findings.push(Finding {
                    rule_id: self.id().into(),
                    title: self.title().into(),
                    severity: Severity::Medium,
                    message: format!(
                        "Function `{}` writes persistent storage without extend_ttl()",
                        function.name
                    ),
                    file: ctx.file.to_string(),
                    line: function.start_line,
                    snippet: snippet_at(ctx.lines, function.start_line),
                    recommendation: "Extend instance or persistent entry TTL after writes to avoid unexpected archival.".into(),
                });
            }
        }
        findings
    }
}

struct UnsafeTemporaryStorageRule;

impl Rule for UnsafeTemporaryStorageRule {
    fn id(&self) -> &'static str {
        "STORE-001"
    }

    fn title(&self) -> &'static str {
        "Temporary storage used for durable protocol state"
    }

    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding> {
        let mut findings = Vec::new();
        let durable_keys = [
            "Admin",
            "Owner",
            "Balance",
            "TotalSupply",
            "Vault",
            "Shares",
        ];
        for (line_no, line) in ctx.lines.iter().enumerate() {
            if !line.contains(".temporary().set(") {
                continue;
            }
            if durable_keys.iter().any(|key| line.contains(key)) {
                findings.push(Finding {
                    rule_id: self.id().into(),
                    title: self.title().into(),
                    severity: Severity::High,
                    message: "Durable protocol state appears to be stored in temporary storage".into(),
                    file: ctx.file.to_string(),
                    line: line_no + 1,
                    snippet: line.trim().to_string(),
                    recommendation: "Use instance or persistent storage for admin balances and vault totals; temporary storage is ledger-scoped.".into(),
                });
            }
        }
        findings
    }
}

struct UncheckedArithmeticRule;

impl Rule for UncheckedArithmeticRule {
    fn id(&self) -> &'static str {
        "ARITH-001"
    }

    fn title(&self) -> &'static str {
        "Unchecked arithmetic that can overflow"
    }

    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding> {
        let ops = [
            "unchecked_add",
            "unchecked_sub",
            "unchecked_mul",
            "unchecked_div",
            "unchecked_rem",
            "unchecked_neg",
            "unchecked_shl",
            "unchecked_shr",
        ];
        let mut findings = Vec::new();
        for (line_no, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            for op in ops {
                if line.contains(op) {
                    findings.push(Finding {
                        rule_id: self.id().into(),
                        title: self.title().into(),
                        severity: Severity::High,
                        message: format!("Uses `{op}`, which silently wraps on overflow"),
                        file: ctx.file.to_string(),
                        line: line_no + 1,
                        snippet: trimmed.to_string(),
                        recommendation: "Use checked_add/checked_sub/checked_mul/checked_div and map None to a typed contract error.".into(),
                    });
                    break;
                }
            }
        }
        findings
    }
}

struct AssertUsageRule;

impl Rule for AssertUsageRule {
    fn id(&self) -> &'static str {
        "ASSERT-001"
    }

    fn title(&self) -> &'static str {
        "Assertion macro aborts the transaction on failure"
    }

    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding> {
        let patterns = [
            ("assert!(", "assert!()"),
            ("assert_eq!(", "assert_eq!()"),
            ("assert_ne!(", "assert_ne!()"),
            ("debug_assert!", "debug_assert!()"),
        ];
        let mut findings = Vec::new();
        for (line_no, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            for (pattern, label) in patterns {
                if line.contains(pattern) {
                    findings.push(Finding {
                        rule_id: self.id().into(),
                        title: self.title().into(),
                        severity: Severity::Medium,
                        message: format!("Uses {label} which panics and aborts the entire transaction"),
                        file: ctx.file.to_string(),
                        line: line_no + 1,
                        snippet: trimmed.to_string(),
                        recommendation: "Validate invariants explicitly and return a typed contract error instead of asserting.".into(),
                    });
                    break;
                }
            }
        }
        findings
    }
}

struct UnsafeBlockRule;

impl Rule for UnsafeBlockRule {
    fn id(&self) -> &'static str {
        "UNSAFE-001"
    }

    fn title(&self) -> &'static str {
        "unsafe block in contract code"
    }

    fn run(&self, ctx: &RuleContext<'_>) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (line_no, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            if line.contains("unsafe {") || line.contains("unsafe{") || line.contains("unsafe fn") {
                findings.push(Finding {
                    rule_id: self.id().into(),
                    title: self.title().into(),
                    severity: Severity::High,
                    message: "Contract code contains an unsafe block; undefined behavior can brick the contract".into(),
                    file: ctx.file.to_string(),
                    line: line_no + 1,
                    snippet: trimmed.to_string(),
                    recommendation: "Eliminate unsafe blocks from on-chain code or isolate them behind a reviewed, documented safety boundary.".into(),
                });
            }
        }
        findings
    }
}

struct FunctionBlock {
    name: String,
    body: String,
    start_line: usize,
}

fn extract_functions(source: &str) -> Vec<FunctionBlock> {
    let mut functions = Vec::new();
    let mut current_name: Option<String> = None;
    let mut start_line = 0;
    let mut brace_depth: i32 = 0;
    let mut body_lines: Vec<String> = Vec::new();
    let mut in_function = false;
    let mut seen_open_brace = false;

    for (idx, line) in source.lines().enumerate() {
        let line_no = idx + 1;
        let opens = line.chars().filter(|c| *c == '{').count() as i32;
        let closes = line.chars().filter(|c| *c == '}').count() as i32;
        let net = opens - closes;

        if !in_function {
            if let Some(name) = parse_fn_name(line) {
                current_name = Some(name);
                start_line = line_no;
                body_lines.clear();
                body_lines.push(line.to_string());
                in_function = true;
                seen_open_brace = opens > 0;
                brace_depth = net;
                if seen_open_brace && brace_depth <= 0 {
                    // Body opened and closed on the signature line itself.
                    if let Some(name) = current_name.take() {
                        functions.push(FunctionBlock {
                            name,
                            body: body_lines.join("\n"),
                            start_line,
                        });
                    }
                    in_function = false;
                }
            }
            continue;
        }

        body_lines.push(line.to_string());
        if !seen_open_brace {
            seen_open_brace = opens > 0;
        }
        brace_depth += net;
        if seen_open_brace && brace_depth <= 0 {
            if let Some(name) = current_name.take() {
                functions.push(FunctionBlock {
                    name,
                    body: body_lines.join("\n"),
                    start_line,
                });
            }
            in_function = false;
        }
    }

    functions
}

fn parse_fn_name(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with("pub fn ") && !trimmed.starts_with("fn ") {
        return None;
    }
    let after_fn = trimmed.split("fn ").nth(1)?;
    let name = after_fn.split(['(', '<']).next()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn looks_like_contract_fn(body: &str) -> bool {
    body.contains("Env") || body.contains("env:") || body.contains("&env")
}

fn is_state_changing(body: &str) -> bool {
    [
        ".set(",
        ".update(",
        ".remove(",
        ".transfer(",
        "transfer(",
        ".mint(",
        ".burn(",
    ]
    .iter()
    .any(|needle| body.contains(needle))
}

fn snippet_at(lines: &[String], line: usize) -> String {
    lines
        .get(line.saturating_sub(1))
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::RuleContext;

    #[test]
    fn auth_rule_flags_missing_require_auth() {
        let source = r#"
pub fn withdraw(env: Env, user: Address, amount: i128) {
    env.storage().instance().set(&DataKey::Balance(user), &amount);
}
"#;
        let lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
        let ctx = RuleContext {
            file: "lib.rs",
            source,
            lines: &lines,
        };
        let findings = AuthMissingRule.run(&ctx);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "AUTH-001");
    }

    #[test]
    fn auth_rule_handles_multiline_signatures() {
        let source = r#"
pub fn withdraw(
    env: Env,
    user: Address,
    amount: i128,
) {
    env.storage().instance().set(&DataKey::Balance(user), &amount);
}
"#;
        let lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
        let ctx = RuleContext {
            file: "lib.rs",
            source,
            lines: &lines,
        };
        let findings = AuthMissingRule.run(&ctx);
        assert_eq!(findings.len(), 1, "multi-line fn body must be analyzed");
        assert_eq!(findings[0].rule_id, "AUTH-001");
        assert_eq!(findings[0].line, 2);
    }

    #[test]
    fn event_rule_handles_multiline_signatures() {
        let source = r#"
pub fn deposit(
    env: Env,
    user: Address,
    amount: i128,
) {
    env.storage().instance().set(&DataKey::Balance(user), &amount);
}
"#;
        let lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
        let ctx = RuleContext {
            file: "lib.rs",
            source,
            lines: &lines,
        };
        let findings = MissingEventsRule.run(&ctx);
        assert_eq!(
            findings.len(),
            1,
            "multi-line fn must be checked for events"
        );
        assert_eq!(findings[0].rule_id, "EVENT-001");
    }

    #[test]
    fn arith_rule_flags_unchecked_arithmetic() {
        let source = r#"
pub fn mint(env: Env, amount: i128) {
    let total = env.total_supply().unchecked_add(amount);
    env.storage().instance().set(&DataKey::Total, &total);
}
"#;
        let lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
        let ctx = RuleContext {
            file: "lib.rs",
            source,
            lines: &lines,
        };
        let findings = UncheckedArithmeticRule.run(&ctx);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "ARITH-001");
        assert_eq!(findings[0].severity, Severity::High);
    }

    #[test]
    fn assert_rule_flags_assertion_macros() {
        let source = r#"
pub fn transfer(env: Env, amount: i128) {
    assert!(amount > 0, "amount must be positive");
    env.storage().instance().set(&DataKey::Balance, &amount);
}
"#;
        let lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
        let ctx = RuleContext {
            file: "lib.rs",
            source,
            lines: &lines,
        };
        let findings = AssertUsageRule.run(&ctx);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "ASSERT-001");
    }

    #[test]
    fn unsafe_rule_flags_unsafe_blocks() {
        let source = r#"
pub fn decode(env: Env, bytes: &[u8]) {
    unsafe { std::mem::transmute::<u32, i32>(0) };
    env.storage().instance().set(&DataKey::Balance, &bytes);
}
"#;
        let lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
        let ctx = RuleContext {
            file: "lib.rs",
            source,
            lines: &lines,
        };
        let findings = UnsafeBlockRule.run(&ctx);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "UNSAFE-001");
        assert_eq!(findings[0].severity, Severity::High);
    }
}
