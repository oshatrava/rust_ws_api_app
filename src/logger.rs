use std::env;

pub fn setup() {
    let app_name = env::var("CARGO_PKG_NAME").unwrap();
    let level = String::from("INFO");
    let env = format!("{app_name}={level},tower_http={level}");

    env::set_var("RUST_LOG", env);
    
    tracing_subscriber::fmt::init();
}