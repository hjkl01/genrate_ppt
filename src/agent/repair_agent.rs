use super::vision_qa::VisionIssue;

pub struct RepairAgent;

impl RepairAgent {
    pub async fn repair(issues: Vec<VisionIssue>) -> Vec<String> {
        issues.into_iter()
            .map(|i| format!("repair: {}", i.description))
            .collect()
    }
}
