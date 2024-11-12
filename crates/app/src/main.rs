use spin_app::App;
use spin_factors_test::build_locked_app;

#[tokio::main]
async fn main() {
    let manifest: toml::value::Table = toml::toml! {
        spin_manifest_version = 2

        [application]
        name = "test-app"

        [[trigger.test-trigger]]
        component = "empty"

        [component.empty]
        source = "does-not-exist.wasm"
    };

    
    let mut locked_app = build_locked_app(&manifest).await.unwrap();
    let app = App::new("test", locked_app);

}
