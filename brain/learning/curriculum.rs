#[derive(Debug, Clone)]
pub struct CurriculumStage {
    pub name: String,
    pub difficulty: f32,
}

#[derive(Debug, Default)]
pub struct Curriculum {
    pub stages: Vec<CurriculumStage>,
    pub current: usize,
}

impl Curriculum {
    pub fn add_stage(&mut self, name: impl Into<String>, difficulty: f32) {
        self.stages.push(CurriculumStage { name: name.into(), difficulty });
    }

    pub fn current_stage(&self) -> Option<&CurriculumStage> { self.stages.get(self.current) }

    pub fn advance(&mut self) -> bool {
        if self.current + 1 < self.stages.len() { self.current += 1; true } else { false }
    }
}
