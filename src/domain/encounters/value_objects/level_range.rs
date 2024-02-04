use crate::{framework::result_loadable::ResultLoadable, infrastructure::results::encounter::LevelRangeResult};

pub struct LevelRange {
    pub min_level: i32,
    pub max_level: i32,
}

impl ResultLoadable<LevelRangeResult> for LevelRange {
    fn create_from_result(result: &LevelRangeResult) -> Self {
        LevelRange { min_level: result.min_level, max_level: result.max_level }
    }
}
