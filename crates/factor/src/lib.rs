use spin_factors::{
    ConfigureAppContext, Factor, FactorInstanceBuilder, FactorRuntimeConfigSource, InitContext, PrepareContext, RuntimeFactors, SelfInstanceBuilder
};


mod host;
mod runtime_config;

/// A factor that provides key-value storage.
#[derive(Default)]
pub struct CounterFactor {
    _priv: (),
}

impl CounterFactor {
    /// Create a new KeyValueFactor.
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl Factor for CounterFactor {
    type RuntimeConfig = ();
    type AppState = AppState;
    type InstanceBuilder = InstanceState;

    fn init<T: Send + 'static>(&mut self, mut ctx: InitContext<T, Self>) -> anyhow::Result<()> {
        ctx.link_bindings(::world::smndtrl::experiments::data::add_to_linker)?;
        Ok(())
    }

    fn configure_app<T: RuntimeFactors>(
        &self,
        mut ctx: ConfigureAppContext<T, Self>,
    ) -> anyhow::Result<Self::AppState> {
        Ok(AppState {})
    }

    fn prepare<T: RuntimeFactors>(
        &self,
        ctx: PrepareContext<T, Self>,
    ) -> anyhow::Result<InstanceState> {
        Ok(InstanceState {
            counters: spin_resource_table::Table::new(5),
        })
    }

}

pub struct AppState {
}

impl AppState {
}

pub struct InstanceState {
    pub counters: spin_resource_table::Table<u8>,
}

impl InstanceState {
}

impl SelfInstanceBuilder for InstanceState {}

