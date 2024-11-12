
use spin_factors::FactorRuntimeConfigSource;
use spin_runtime_config::TomlRuntimeConfigSource;

use crate::CounterFactor;


impl FactorRuntimeConfigSource<CounterFactor> for TomlRuntimeConfigSource<'_, '_> {
    fn get_runtime_config(&mut self) -> anyhow::Result<Option<()>> {
        Ok(Some(()))
    }
}