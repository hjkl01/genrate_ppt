use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutPatch {
    pub element_id: String,
    pub action: String,
    pub value: String,
}

#[derive(Debug, Clone, Default)]
pub struct PatchEngine {
    pub patches: Vec<LayoutPatch>,
}

impl PatchEngine {
    pub fn apply(&mut self, patch: LayoutPatch) {
        self.patches.push(patch);
    }
}
