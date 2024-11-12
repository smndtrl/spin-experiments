use std::path::PathBuf;

use anyhow::Context;
// use product_factor_development::DevelopmentFactor;
use ::factor::CounterFactor;
use spin_factor_key_value::KeyValueFactor;
use spin_factor_outbound_http::OutboundHttpFactor;
use spin_factor_outbound_networking::OutboundNetworkingFactor;
use spin_factor_variables::VariablesFactor;
use spin_factor_wasi::{spin::SpinFilesMounter, WasiFactor};
use spin_factors::{FactorRuntimeConfigSource, RuntimeFactors};
use spin_factors_executor::FactorsExecutor;
use spin_runtime_config::{ResolvedRuntimeConfig, TomlRuntimeConfigSource};
use spin_runtime_factors::{TriggerAppArgs, TriggerFactors, TriggerFactorsRuntimeConfig};
use spin_trigger::cli::{FactorsConfig, InitialKvSetterHook, KeyValueDefaultStoreSummaryHook, RuntimeFactorsBuilder, SqlStatementExecutorHook, SqliteDefaultStoreSummaryHook, StdioLoggingExecutorHooks};


#[derive(RuntimeFactors)]
pub struct CustomTriggerFactors {
    pub wasi: WasiFactor,
    pub variables: VariablesFactor,
    pub key_value: KeyValueFactor,
    pub outbound_networking: OutboundNetworkingFactor,
    pub outbound_http: OutboundHttpFactor,
    pub counter: CounterFactor,
}

impl CustomTriggerFactors {
    pub fn new(
        state_dir: Option<PathBuf>,
        working_dir: impl Into<PathBuf>,
        allow_transient_writes: bool,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            wasi: wasi_factor(working_dir, allow_transient_writes),
            variables: VariablesFactor::default(),
            key_value: KeyValueFactor::new(),
            outbound_networking: outbound_networking_factor(),
            outbound_http: OutboundHttpFactor::default(),
            counter: CounterFactor::new(),
        })
    }
}

// impl RuntimeFactors for CustomTriggerFactors {

// }

fn wasi_factor(working_dir: impl Into<PathBuf>, allow_transient_writes: bool) -> WasiFactor {
    WasiFactor::new(SpinFilesMounter::new(working_dir, allow_transient_writes))
}

fn outbound_networking_factor() -> OutboundNetworkingFactor {
    fn disallowed_host_handler(scheme: &str, authority: &str) {
        let host_pattern = format!("{scheme}://{authority}");
        tracing::error!("Outbound network destination not allowed: {host_pattern}");
        if scheme.starts_with("http") && authority == "self" {
            println!("A component tried to make an HTTP request to its own app but it does not have permission.");
        } else {
            println!(
                "A component tried to make an outbound network connection to disallowed destination '{host_pattern}'."
            );
        };
        eprintln!("To allow this request, add 'allowed_outbound_hosts = [\"{host_pattern}\"]' to the manifest component section.");
    }

    let mut factor = OutboundNetworkingFactor::new();
    factor.set_disallowed_host_handler(disallowed_host_handler);
    factor
}

pub struct CustomFactorsBuilder;

impl RuntimeFactorsBuilder for CustomFactorsBuilder {
    type CliArgs = TriggerAppArgs;
    type Factors = CustomTriggerFactors;
    type RuntimeConfig = ResolvedRuntimeConfig<CustomTriggerFactorsRuntimeConfig>;

    fn build(
        config: &FactorsConfig,
        args: &Self::CliArgs,
    ) -> anyhow::Result<(Self::Factors, Self::RuntimeConfig)> {
        let runtime_config = ResolvedRuntimeConfig::<CustomTriggerFactorsRuntimeConfig>::from_file(
            config.runtime_config_file.clone().as_deref(),
            config.local_app_dir.clone().map(PathBuf::from),
            config.state_dir.clone(),
            config.log_dir.clone(),
        )?;

        runtime_config.summarize(config.runtime_config_file.as_deref());

        let factors = CustomTriggerFactors::new(
            runtime_config.state_dir(),
            config.working_dir.clone(),
            args.allow_transient_write,
        )
        .context("failed to create factors")?;
        Ok((factors, runtime_config))
    }

    fn configure_app<U: Send + 'static>(
        executor: &mut FactorsExecutor<Self::Factors, U>,
        runtime_config: &Self::RuntimeConfig,
        config: &FactorsConfig,
        args: &Self::CliArgs,
    ) -> anyhow::Result<()> {
        executor.add_hooks(StdioLoggingExecutorHooks::new(
            config.follow_components.clone(),
            runtime_config.log_dir(),
        ));
        executor.add_hooks(SqlStatementExecutorHook::new(
            args.sqlite_statements.clone(),
        ));
        executor.add_hooks(InitialKvSetterHook::new(args.key_values.clone()));
        executor.add_hooks(SqliteDefaultStoreSummaryHook);
        executor.add_hooks(KeyValueDefaultStoreSummaryHook);
        Ok(())
    }
}

impl From<ResolvedRuntimeConfig<CustomTriggerFactorsRuntimeConfig>> for CustomTriggerFactorsRuntimeConfig {
    fn from(value: ResolvedRuntimeConfig<CustomTriggerFactorsRuntimeConfig>) -> Self {
        value.runtime_config
    }
}

impl TryFrom<TomlRuntimeConfigSource<'_, '_>> for CustomTriggerFactorsRuntimeConfig {
    type Error = anyhow::Error;

    fn try_from(value: TomlRuntimeConfigSource<'_, '_>) -> Result<Self, Self::Error> {
        Self::from_source(value)
    }
}

