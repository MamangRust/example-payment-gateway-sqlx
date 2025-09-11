mod auth;
mod card;
mod merchant;
mod role;
mod saldo;
mod topup;
mod transaction;
mod transfer;
mod user;
mod withdraw;

use crate::state::AppState;
use anyhow::Result;
use axum::extract::DefaultBodyLimit;
use shared::utils::shutdown_signal;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::limit::RequestBodyLimitLayer;
use utoipa::{Modify, OpenApi, openapi::security::SecurityScheme};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub use self::auth::auth_routes;
pub use self::card::card_routes;
pub use self::merchant::merchant_routes;
pub use self::role::role_routes;
pub use self::saldo::saldo_routes;
pub use self::topup::topup_routes;
pub use self::transaction::transaction_routes;
pub use self::transfer::transfer_routes;
pub use self::user::user_routes;
pub use self::withdraw::withdraw_routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register_user_handler,
        auth::login_user_handler,
        auth::get_me_handler,
        auth::refresh_token_handler,

        card::get_cards,
        card::create_card,
        card::get_active_cards,
        card::get_trashed_cards,
        card::get_card,
        card::update_card,
        card::trash_card_handler,
        card::restore_card_handler,
        card::delete_card,
        card::restore_all_card_handler,
        card::delete_all_card_handler,
        card::get_monthly_balance,
        card::get_yearly_balance,
        card::get_monthly_topup_amount,
        card::get_yearly_topup_amount,
        card::get_monthly_transaction_amount,
        card::get_yearly_transaction_amount,
        card::get_monthly_transfer_amount,
        card::get_yearly_transfer_amount,
        card::get_monthly_withdraw_amount,
        card::get_yearly_withdraw_amount,
        card::get_monthly_balance_by_card,
        card::get_yearly_balance_by_card,
        card::get_monthly_topup_amount_by_card,
        card::get_yearly_topup_amount_by_card,
        card::get_monthly_transaction_amount_by_card,
        card::get_yearly_transaction_amount_by_card,
        card::get_monthly_transfer_amount_by_card,
        card::get_yearly_transfer_amount_by_card,
        card::get_monthly_withdraw_amount_by_card,
        card::get_yearly_withdraw_amount_by_card,
        card::get_card_dashboard,
        card::get_card_dashboard_by_card_number,

        merchant::get_merchants,
        merchant::create_merchant,
        merchant::get_active_merchants,
        merchant::get_trashed_merchants,
        merchant::get_merchant,
        merchant::update_merchant,
        merchant::trash_merchant_handler,
        merchant::restore_merchant_handler,
        merchant::delete_merchant,
        merchant::restore_all_merchant_handler,
        merchant::delete_all_merchant_handler,
        merchant::get_monthly_amount,
        merchant::get_yearly_amount,
        merchant::get_monthly_method,
        merchant::get_yearly_method,
        merchant::get_monthly_total_amount,
        merchant::get_yearly_total_amount,
        merchant::get_monthly_amount_by_merchant,
        merchant::get_yearly_amount_by_merchant,
        merchant::get_monthly_method_by_merchant,
        merchant::get_yearly_method_by_merchant,
        merchant::get_monthly_total_amount_by_merchant,
        merchant::get_yearly_total_amount_by_merchant,
        merchant::get_monthly_amount_by_apikey,
        merchant::get_yearly_amount_by_apikey,
        merchant::get_monthly_method_by_apikey,
        merchant::get_yearly_method_by_apikey,
        merchant::get_monthly_total_amount_by_apikey,
        merchant::get_yearly_total_amount_by_apikey,
        merchant::get_merchant_transactions,
        merchant::get_merchant_transactions_by_id,
        merchant::get_merchant_transactions_by_apikey,

        role::get_roles,
        role::get_active_roles,
        role::get_trashed_roles,
        role::get_role,
        role::get_roles_by_user_id,
        role::create_role,
        role::update_role,
        role::trash_role_handler,
        role::restore_role_handler,
        role::delete_role,
        role::restore_all_role_handler,
        role::delete_all_role_handler,

        saldo::get_saldos,
        saldo::get_active_saldos,
        saldo::get_trashed_saldos,
        saldo::get_saldo,
        saldo::create_saldo,
        saldo::update_saldo,
        saldo::trash_saldo_handler,
        saldo::restore_saldo_handler,
        saldo::delete_saldo,
        saldo::restore_all_saldo_handler,
        saldo::delete_all_saldo_handler,
        saldo::get_monthly_balance,
        saldo::get_yearly_balance,
        saldo::get_monthly_total_balance,
        saldo::get_yearly_total_balance,

        topup::get_topups,
        topup::get_topups_by_card_number,
        topup::get_active_topups,
        topup::get_trashed_topups,
        topup::get_topup,
        topup::create_topup,
        topup::update_topup,
        topup::trash_topup_handler,
        topup::restore_topup_handler,
        topup::delete_topup,
        topup::restore_all_topup_handler,
        topup::delete_all_topup_handler,
        topup::get_monthly_topup_amounts,
        topup::get_yearly_topup_amounts,
        topup::get_monthly_topup_methods,
        topup::get_yearly_topup_methods,
        topup::get_month_topup_status_success,
        topup::get_yearly_topup_status_success,
        topup::get_month_topup_status_failed,
        topup::get_yearly_topup_status_failed,
        topup::get_monthly_topup_amounts_by_card,
        topup::get_yearly_topup_amounts_by_card,
        topup::get_monthly_topup_methods_by_card,
        topup::get_yearly_topup_methods_by_card,
        topup::get_month_topup_status_success_by_card,
        topup::get_yearly_topup_status_success_by_card,
        topup::get_month_topup_status_failed_by_card,
        topup::get_yearly_topup_status_failed_by_card,


        transaction::get_transactions,
        transaction::get_transactions_by_card_number,
        transaction::get_active_transactions,
        transaction::get_trashed_transactions,
        transaction::get_transaction,
        transaction::get_transactions_by_merchant_id,
        transaction::create_transaction,
        transaction::update_transaction,
        transaction::trash_transaction_handler,
        transaction::restore_transaction_handler,
        transaction::delete_transaction,
        transaction::restore_all_transaction_handler,
        transaction::delete_all_transaction_handler,
        transaction::get_monthly_amounts,
        transaction::get_yearly_amounts,
        transaction::get_monthly_method,
        transaction::get_yearly_method,
        transaction::get_month_status_success,
        transaction::get_yearly_status_success,
        transaction::get_month_status_failed,
        transaction::get_yearly_status_failed,
        transaction::get_monthly_amounts_by_card,
        transaction::get_yearly_amounts_by_card,
        transaction::get_monthly_method_by_card,
        transaction::get_yearly_method_by_card,
        transaction::get_month_status_success_by_card,
        transaction::get_yearly_status_success_by_card,
        transaction::get_month_status_failed_by_card,
        transaction::get_yearly_status_failed_by_card,


        transfer::get_transfers,
        transfer::get_transfer,
        transfer::get_active_transfers,
        transfer::get_trashed_transfers,
        transfer::get_transfers_by_transfer_from,
        transfer::get_transfers_by_transfer_to,
        transfer::create_transfer,
        transfer::update_transfer,
        transfer::trash_transfer_handler,
        transfer::restore_transfer_handler,
        transfer::delete_transfer,
        transfer::restore_all_transfer_handler,
        transfer::delete_all_transfer_handler,
        transfer::get_monthly_amounts,
        transfer::get_yearly_amounts,
        transfer::get_month_status_success,
        transfer::get_yearly_status_success,
        transfer::get_month_status_failed,
        transfer::get_yearly_status_failed,
        transfer::get_monthly_amounts_by_sender,
        transfer::get_monthly_amounts_by_receiver,
        transfer::get_yearly_amounts_by_sender,
        transfer::get_yearly_amounts_by_receiver,
        transfer::get_month_status_success_by_card,
        transfer::get_yearly_status_success_by_card,
        transfer::get_month_status_failed_by_card,
        transfer::get_yearly_status_failed_by_card,

        user::get_users,
        user::get_user,
        user::get_active_users,
        user::get_trashed_users,
        user::create_user,
        user::update_user,
        user::trash_user_handler,
        user::restore_user_handler,
        user::delete_user,
        user::restore_all_user_handler,
        user::delete_all_user_handler,


        withdraw::get_withdraws,
        withdraw::get_withdraws_by_card_number,
        withdraw::get_withdraw,
        withdraw::get_active_withdraws,
        withdraw::get_trashed_withdraws,
        withdraw::create_withdraw,
        withdraw::update_withdraw,
        withdraw::trash_withdraw_handler,
        withdraw::restore_withdraw_handler,
        withdraw::delete_withdraw,
        withdraw::restore_all_withdraw_handler,
        withdraw::delete_all_withdraw_handler,
        withdraw::get_monthly_withdraws,
        withdraw::get_yearly_withdraws,
        withdraw::get_month_status_success,
        withdraw::get_yearly_status_success,
        withdraw::get_month_status_failed,
        withdraw::get_yearly_status_failed,
        withdraw::get_monthly_by_card_number,
        withdraw::get_yearly_by_card_number,
        withdraw::get_month_status_success_by_card,
        withdraw::get_yearly_status_success_by_card,
        withdraw::get_month_status_failed_by_card,
        withdraw::get_yearly_status_failed_by_card,
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "Authentication and authorization endpoints"),
        (name = "Role", description = "Role and permissions management endpoints"),
        (name = "User", description = "User management and profile endpoints"),
        (name = "Card", description = "Card management and statistics endpoints"),
        (name = "Merchant", description = "Merchant account and business endpoints"),
        (name = "Saldo", description = "Balance inquiry and saldo operations"),
        (name = "Topup", description = "Top-up and funding endpoints"),
        (name = "Transaction", description = "Transaction processing and history endpoints"),
        (name = "Transfer", description = "Money transfer between accounts or cards"),
        (name = "Withdraw", description = "Withdraw operations and endpoints"),
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
    }
}

pub struct AppRouter;

impl AppRouter {
    pub async fn serve(port: u16, app_state: AppState) -> Result<()> {
        let shared_state = Arc::new(app_state);

        let api_router = OpenApiRouter::with_openapi(ApiDoc::openapi())
            .with_state(shared_state.clone())
            .merge(auth_routes(shared_state.clone()))
            .merge(user_routes(shared_state.clone()))
            .merge(role_routes(shared_state.clone()))
            .merge(card_routes(shared_state.clone()))
            .merge(merchant_routes(shared_state.clone()))
            .merge(saldo_routes(shared_state.clone()))
            .merge(topup_routes(shared_state.clone()))
            .merge(transaction_routes(shared_state.clone()))
            .merge(transfer_routes(shared_state.clone()))
            .merge(withdraw_routes(shared_state.clone()));

        let router_with_layers = api_router
            .layer(DefaultBodyLimit::disable())
            .layer(RequestBodyLimitLayer::new(250 * 1024 * 1024));

        let (app_router, api) = router_with_layers.split_for_parts();

        let app = app_router
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

        let addr = format!("0.0.0.0:{port}");
        let listener = TcpListener::bind(&addr).await?;

        println!("🚀 Server running on http://{}", listener.local_addr()?);
        println!("📚 API Documentation available at:");
        println!("   📖 Swagger UI: http://localhost:{port}/swagger-ui");

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();

        Ok(())
    }
}
