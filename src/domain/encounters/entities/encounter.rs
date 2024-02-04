use crate::domain::encounters::entities::enemy_formula::EnemyFormula;
use crate::domain::encounters::value_objects::level_range::LevelRange;
use crate::domain::encounters::value_objects::roll_range::RollRange;

pub struct Encounter {
    pub environment: &'static str,
    pub level_range: &'static LevelRange,
    pub roll_range: &'static RollRange,
}

impl Encounter {
    pub fn new(environment: &str, level_range: &LevelRange, roll_range: &RollRange) -> Self {
        Encounter {
            environment,
            level_range,
            roll_range,
        }
    }
}

pub trait BaseEncounter {
    fn environment(&self) -> &str;
    fn level_range(&self) -> &LevelRange;
    fn roll_range(&self) -> &RollRange;
}

impl BaseEncounter for Encounter {
    fn environment(&self) -> &str {
        &self.environment
    }

    fn level_range(&self) -> &LevelRange {
        &self.level_range
    }

    fn roll_range(&self) -> &RollRange {
        &self.roll_range
    }
}

pub struct SituationalEncounter {
    pub encounter: Encounter,
    pub situation: str,
}

pub struct CombatEncounter {
    pub encounter: Encounter,
    pub enemy_formulas: Vec<EnemyFormula>,
}

impl BaseEncounter for SituationalEncounter {
    fn environment(&self) -> &str {
        &self.encounter.environment
    }

    fn level_range(&self) -> &LevelRange {
        &self.encounter.level_range
    }

    fn roll_range(&self) -> &RollRange {
        &self.encounter.roll_range
    }
}

impl BaseEncounter for CombatEncounter {
    fn environment(&self) -> &str {
        &self.encounter.environment
    }

    fn level_range(&self) -> &LevelRange {
        &self.encounter.level_range
    }

    fn roll_range(&self) -> &RollRange {
        &self.encounter.roll_range
    }
}
