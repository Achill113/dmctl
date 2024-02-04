use crate::{framework::result_loadable::ResultLoadable, infrastructure::results::encounter::RollRangeResult};

pub struct RollRange {
    pub min_roll: i32,
    pub max_roll: i32,
}

impl ResultLoadable<RollRangeResult> for RollRange {
    fn create_from_result(result: &RollRangeResult) -> Self {
        RollRange { min_roll: result.min_roll, max_roll: result.max_roll }
    }
}
