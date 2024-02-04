use crate::{framework::result_loadable::ResultLoadable, infrastructure::results::encounter::EnemyFormulaResult};

pub struct EnemyFormula {
    pub enemy_name: String,
    pub die_type: Option<String>,
    pub count: i32,
}

impl ResultLoadable<EnemyFormulaResult> for EnemyFormula {
    fn create_from_result(result: &EnemyFormulaResult) -> Self {
        EnemyFormula { enemy_name: result.enemy_name.clone(), die_type: result.die_type.clone(), count: result.count }
    }
}
