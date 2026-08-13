pub struct RepairAgent;

impl RepairAgent {
    pub fn repair_instruction(&self, issues: &[String]) -> String {
        if issues.is_empty() {
            return "No repair required".into();
        }

        format!("Please repair slides: {}", issues.join(", "))
    }
}
