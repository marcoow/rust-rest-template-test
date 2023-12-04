use axum_on_rails::{
    load_config,
    test::helpers::{build_test_context, prepare_db, TestContext},
    Environment,
};
use my_app_config::Config;
use my_app_web::routes::routes;
use my_app_web::state::AppState;
use sqlx::postgres::PgPoolOptions;
use std::cell::OnceCell;

pub async fn setup() -> TestContext {
    let init_config: OnceCell<Config> = OnceCell::new();
    let config = init_config.get_or_init(|| load_config(&Environment::Test));

    let db_config = prepare_db(&config.database).await;
    let db_pool = PgPoolOptions::new()
        .connect_with(db_config.clone())
        .await
        .expect("Could not connect to database!");

    let app = routes(AppState {
        db_pool: db_pool.clone(),
    });

    build_test_context(app, db_pool, db_config)
}
