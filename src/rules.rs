use jj_lib::repo::ReadonlyRepo;
use serde_json;

// Trait for a rule that can be run against a repo and store results
pub trait RepoRule {
    /// Run the rule, storing results internally.
    fn run(&mut self, repo: &ReadonlyRepo);

    /// Print results recursively (to console, for now).
    fn print_results(&self, indent: usize);

    /// Optionally: Serialize results to JSON (stub for now).
    fn to_json(&self) -> serde_json::Value;
}

// Composite rule that holds multiple rules and runs them recursively
pub struct CompositeRepoRule {
    pub rules: Vec<Box<dyn RepoRule>>,
}

impl CompositeRepoRule {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Box<dyn RepoRule>) {
        self.rules.push(rule);
    }
}

impl RepoRule for CompositeRepoRule {
    fn run(&mut self, repo: &ReadonlyRepo) {
        for rule in &mut self.rules {
            rule.run(repo);
        }
    }

    fn print_results(&self, indent: usize) {
        let pad = " ".repeat(indent);
        println!("{}CompositeRule:", pad);
        for rule in &self.rules {
            rule.print_results(indent + 2);
        }
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self.rules.iter().map(|c| c.to_json()).collect::<Vec<_>>())
    }
}

// Example: A dummy rule implementation
pub struct DummyRepoRule {
    pub name: String,
    pub result: Option<String>,
}

impl DummyRepoRule {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            result: None,
        }
    }
}

impl RepoRule for DummyRepoRule {
    fn run(&mut self, _repo: &ReadonlyRepo) {
        // Example: just set a dummy result
        self.result = Some(format!("Rule '{}' passed", self.name));
    }

    fn print_results(&self, indent: usize) {
        let pad = " ".repeat(indent);
        println!("{}DummyRule '{}': {:?}", pad, self.name, self.result);
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "result": self.result,
        })
    }
}
