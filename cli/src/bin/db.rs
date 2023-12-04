use axum_on_rails::{cli::db::cli, load_config};
use my_app_config::Config;

#[tokio::main]
async fn main() {
    cli(|env| {
        let config: Config = load_config(&env);
        config.database
    })
    .await;
}
