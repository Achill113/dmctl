use crate::domain::encounters::entities::enemy_formula::EnemyFormula;
use crate::domain::encounters::value_objects::level_range::LevelRange;
use crate::domain::encounters::value_objects::roll_range::RollRange;

pub struct Encounter {
    pub environment: String,
    pub level_range: LevelRange,
    pub roll_range: RollRange,
}

impl Encounter {
    pub fn new(environment: String, level_range: LevelRange, roll_range: RollRange) -> Self {
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
    pub situation: String,
}

impl SituationalEncounter {
    pub fn new(encounter: Encounter, situation: String) -> Self {
        SituationalEncounter { encounter, situation }
    }
}

pub struct CombatEncounter {
    pub encounter: Encounter,
    pub enemy_formulas: Vec<EnemyFormula>,
}

impl CombatEncounter {
    pub fn new(encounter: Encounter, enemy_formulas: Vec<EnemyFormula>) -> Self {
        CombatEncounter { encounter, enemy_formulas }
    }
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
