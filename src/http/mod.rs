use axum::{
    routing::{get, patch, post},
    Router,
};
mod activity_log_retrieval;
mod auto_swap_service;
mod health_check;
mod percentage_update;
mod subscription;
mod transaction_logs;
mod types;
pub use types::is_valid_address;
mod unsubscription;
use crate::AppState;

// Application router.
// All routes should be merged here.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check::health_check))
        .route(
            "/log_transaction",
            post(transaction_logs::log_transaction_to_db),
        )
        .route("/unsubscribe", post(unsubscription::handle_unsubscribe))
        .route(
            "/subscriptions",
            get(subscription::get_subscription).post(subscription::create_subscription),
        )
        .route("/log_retrieval", get(activity_log_retrieval::log_retrieval))
        .route(
            "/update_percentage",
            patch(percentage_update::update_percentage),
        )
        .route("/auto_swap", post(auto_swap_service::handle_auto_swap))
}
