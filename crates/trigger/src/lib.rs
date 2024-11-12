use spin_factors::{anyhow, RuntimeFactors};
use spin_trigger::{
    cli::{FactorsTriggerCommand, NoCliArgs},
    Trigger,
};

mod factors;
pub use factors::CustomFactorsBuilder;

/// A [`spin_trigger::TriggerApp`] for the HTTP trigger.
pub(crate) type TriggerApp<F> = spin_trigger::TriggerApp<LineTrigger, F>;

/// A [`spin_trigger::TriggerInstanceBuilder`] for the HTTP trigger.
pub(crate) type TriggerInstanceBuilder<'a, F> =
    spin_trigger::TriggerInstanceBuilder<'a, LineTrigger, F>;

wasmtime::component::bindgen!({
    path: "../../wit",
    world: "counter",
    async: true
});

pub struct LineTrigger {}

impl<F: RuntimeFactors> Trigger<F> for LineTrigger {
    const TYPE: &'static str = "http";

    type CliArgs = NoCliArgs;
    type InstanceState = ();

    fn new(cli_args: Self::CliArgs, app: &spin_app::App) -> anyhow::Result<Self> {
        Self::new()
    }

    async fn run(self, trigger_app: TriggerApp<F>) -> anyhow::Result<()> {
        let instance_builder = trigger_app.prepare("store")?;
        let (instance, mut store) = instance_builder.instantiate(()).await?;
        let counter = Counter::new(&mut store, &instance)?;
        let _resource = counter
            .smndtrl_experiments_component()
            .call_modify(&mut store)
            .await
            .unwrap();

        Ok(())
    }
}

impl LineTrigger {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
