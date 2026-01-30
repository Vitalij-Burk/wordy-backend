use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::{
    api::handlers::{
        translate_handlers::translate,
        user_handlers::{
            create_user, delete_user_by_id, get_user_by_id, get_user_by_key, update_user_by_id,
        },
        word_pair_handlers::{
            add_word_pair_by_user_id, add_word_pair_by_user_key, delete_word_pair_by_id,
            get_word_pair_by_id, get_word_pairs_by_user_id, get_word_pairs_by_user_key,
        },
    },
    application::services::{
        translate_service::TranslateService, user_service::UserService,
        word_pair_service::WordPairService,
    },
    domain::traits::repositories::repository::Repository,
    infrastructure::{
        external_api::translate::translate::TranslatorsTranslator,
        storage::database::repositories::{
            user_repository::UserPostgresRepository,
            word_pair_repository::WordPairPostgresRepository,
        },
    },
};

mod api;
mod application;
mod domain;
mod infrastructure;

#[derive(Clone)]
pub struct AppState {
    pub translate_service: TranslateService<TranslatorsTranslator>,
    pub user_service: UserService<UserPostgresRepository>,
    pub word_pair_service: WordPairService<WordPairPostgresRepository>,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        let user_repo = UserPostgresRepository::new(db.clone());
        let word_pair_repo = WordPairPostgresRepository::new(db.clone());
        let translator = TranslatorsTranslator;

        let user_service = UserService::new(user_repo);
        let word_pair_service = WordPairService::new(word_pair_repo);
        let translate_service = TranslateService::new(translator);

        Self {
            translate_service: translate_service,
            user_service: user_service,
            word_pair_service: word_pair_service,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();
    let pool = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;
    let state = AppState::new(pool);
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/user/create/", post(create_user))
        .route("/translate/", post(translate))
        .route("/user/id/{id}/", get(get_user_by_id))
        .route("/user/key/{key}/", get(get_user_by_key))
        .route("/user/update/id/{id}/", post(update_user_by_id))
        .route("/user/delete/id/{id}/", post(delete_user_by_id))
        .route(
            "/user/user_id/{user_id}/wordpair/create/",
            post(add_word_pair_by_user_id),
        )
        .route(
            "/user/key/{key}/wordpair/create/",
            post(add_word_pair_by_user_key),
        )
        .route(
            "/user/user_id/{user_id}/wordpairs/",
            get(get_word_pairs_by_user_id),
        )
        .route(
            "/user/key/{key}/wordpairs/",
            get(get_word_pairs_by_user_key),
        )
        .route("/wordpair/id/{id}/", get(get_word_pair_by_id))
        .route("/wordpair/delete/id/{id}/", post(delete_word_pair_by_id))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
