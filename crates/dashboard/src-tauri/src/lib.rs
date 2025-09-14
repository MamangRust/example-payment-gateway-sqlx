use command::auth_command::{get_me, login, myrefresh_token, register};
use command::user::user_command::{
    create_user, find_all_users, find_by_active, find_by_id_user, trashed_user, update_user,
};

use command::user::user_trashed_command::{
    delete_permanent_all_user, delete_permanent_user, find_all_users_trashed,
    restore_user_all_trashed, restore_user_trashed,
};

use command::role::role_command::{
    create_role, find_all_roles, find_by_active_role, find_by_id_role, trashed_role, update_role,
};

use command::role::role_trashed_command::{
    delete_permanent_all_role, delete_permanent_role, find_all_roles_trashed,
    restore_role_all_trashed, restore_role_trashed,
};

use command::card::card_command::{
    create_card, find_active_card, find_all_card, find_by_card_number, find_by_id_card,
    find_by_user, find_dashboard, find_dashboard_by_card_number, find_month_balance_by_card_card,
    find_month_balance_card, find_month_topup_amount_by_card_card, find_month_topup_amount_card,
    find_month_transaction_amount_by_card_card, find_month_transaction_amount_card,
    find_month_transfer_receiver_amount_by_card_card, find_month_transfer_receiver_amount_card,
    find_month_transfer_sender_amount_by_card_card, find_month_transfer_sender_amount_card,
    find_month_withdraw_amount_by_card_card, find_month_withdraw_amount_card,
    find_year_balance_by_card_card, find_year_balance_card, find_year_topup_amount_by_card_card,
    find_year_transfer_receiver_amount_by_card_card, find_year_transfer_sender_amount_by_card_card,
    findyear_topup_amount_card, findyear_transaction_amount_by_card_card,
    findyear_transaction_amount_card, findyear_transfer_receiver_amount_card,
    findyear_transfer_sender_amount_card, findyear_withdraw_amount_by_card_card,
    findyear_withdraw_amount_card, trashed_card, update_card,
};
use command::card::card_trashed_command::{
    delete_permanent_all_cards, delete_permanent_card, find_all_cards_trashed,
    restore_all_cards_trashed, restore_card_trashed,
};

use command::merchant::merchant_command::{
    create_merchant, find_all_merchants, find_all_transactions, find_all_transactions_by_api_key,
    find_all_transactions_by_merchant, find_by_active_merchant, find_by_api_key, find_by_id,
    find_by_merchant_user, find_month_amount, find_month_amount_by_merchant,
    find_month_payment_method, find_month_payment_method_by_merchant, find_month_total_amount,
    find_month_total_amount_by_merchant, find_year_amount, find_year_amount_by_merchant,
    find_year_payment_method, find_year_payment_method_by_merchant, find_year_total_amount,
    find_year_total_amount_by_merchant, trashed_merchant, update_merchant,
};

use command::merchant::merchant_trashed_command::{
    delete_permanent_all_merchant, delete_permanent_merchant, find_all_merchants_trashed,
    restore_merchant_all_trashed, restore_merchant_trashed,
};

use command::saldo::saldo_command::{
    create_saldo, find_all_saldos, find_by_active_saldo, find_by_id_saldo, find_month_balance,
    find_month_total_balance, find_year_balance, find_year_total_balance, trashed_saldo,
    update_saldo,
};

use command::saldo::saldo_trashed_command::{
    delete_permanent_all_saldo, delete_permanent_saldo, find_all_saldos_trashed,
    restore_saldo_all_trashed, restore_saldo_trashed,
};

use command::topup::topup_command::{
    create_topup, find_all_topups, find_by_active_topup, find_by_card_number_topup,
    find_by_id_topup, find_month_status_failed, find_month_status_failed_by_card_number,
    find_month_status_success, find_month_status_success_by_card_number, find_month_topup_amount,
    find_month_topup_amount_by_card, find_month_topup_method, find_month_topup_method_by_card,
    find_year_status_failed, find_year_status_failed_by_card_number, find_year_status_success,
    find_year_status_success_by_card_number, find_year_topup_amount,
    find_year_topup_amount_by_card, find_year_topup_method, find_year_topup_method_by_card,
    trashed_topup, update_topup,
};

use command::topup::topup_trashed_command::{
    delete_permanent_all_topup, delete_permanent_topup, find_all_topups_trashed,
    restore_topup_all_trashed, restore_topup_trashed,
};

use command::transaction::transaction_command::{
    create_transaction, find_all_transactions_transaction, find_by_active_transaction,
    find_by_card_number_transaction, find_by_id_transaction, find_by_merchant_transaction,
    find_month_status_failed_transaction, find_month_status_failed_transaction_by_card_number,
    find_month_status_success_transaction, find_month_status_success_transaction_by_card_number,
    find_month_transaction_amount, find_month_transaction_amount_by_card,
    find_month_transaction_method, find_month_transaction_method_by_card,
    find_year_status_failed_transaction, find_year_status_failed_transaction_by_card_number,
    find_year_status_success_transaction, find_year_status_success_transaction_by_card_number,
    find_year_transaction_amount, find_year_transaction_amount_by_card,
    find_year_transaction_method, find_year_transaction_method_by_card, trashed_transaction,
    update_transaction,
};

use command::transaction::transaction_trashed_command::{
    delete_permanent_all_transaction, delete_permanent_transaction, find_all_transactions_trashed,
    restore_transaction_all_trashed, restore_transaction_trashed,
};

use command::transfer::transfer_command::{
    create_transfer, find_all_transfers, find_by_active_transfer, find_by_id_transfer,
    find_by_transfer_from, find_by_transfer_to, find_month_status_failed_transfer,
    find_month_status_failed_transfer_by_card_number, find_month_status_success_transfer,
    find_month_status_success_transfer_by_card_number, find_month_transfer_amount_by_receiver,
    find_month_transfer_amount_by_sender, find_month_transfer_amount_transfer,
    find_year_status_failed_transfer, find_year_status_failed_transfer_by_card_number,
    find_year_status_success_transfer, find_year_status_success_transfer_by_card_number,
    find_year_transfer_amount_by_receiver, find_year_transfer_amount_by_sender,
    find_year_transfer_amount_transfer, trashed_transfer, update_transfer,
};

use command::transfer::transfer_trashed_command::{
    delete_permanent_all_transfer, delete_permanent_transfer, find_all_transfers_trashed,
    restore_transfer_all_trashed, restore_transfer_trashed,
};

use command::withdraw::withdraw_command::{
    create_withdraw, find_all_withdraws, find_by_active_withdraw, find_by_card_number_withdraw,
    find_by_id_withdraw, find_month_status_failed_withdraw,
    find_month_status_failed_withdraw_by_card_number, find_month_status_success_withdraw,
    find_month_status_success_withdraw_by_card_number, find_month_withdraw_amount_by_card_withdraw,
    find_month_withdraw_amount_withdraw, find_year_status_failed_withdraw,
    find_year_status_failed_withdraw_by_card_number, find_year_status_success_withdraw,
    find_year_status_success_withdraw_by_card_number, find_year_withdraw_amount,
    find_year_withdraw_amount_by_card, trashed_withdraw, update_withdraw,
};

use command::withdraw::withdraw_trashed_command::{
    delete_permanent_all_withdraw, delete_permanent_withdraw, find_all_withdraws_trashed,
    restore_withdraw_all_trashed, restore_withdraw_trashed,
};
use helpers::log_tracing;

mod command;
mod domain;
mod helpers;
mod model;
mod service;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    log_tracing::tracing();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            login,
            register,
            get_me,
            myrefresh_token,
            create_user,
            find_all_users,
            find_by_active,
            find_by_id_user,
            trashed_user,
            update_user,
            delete_permanent_all_user,
            delete_permanent_user,
            find_all_users_trashed,
            restore_user_all_trashed,
            restore_user_trashed,
            create_role,
            find_all_roles,
            find_by_active_role,
            find_by_id_role,
            trashed_role,
            update_role,
            delete_permanent_all_role,
            delete_permanent_role,
            find_all_roles_trashed,
            restore_role_all_trashed,
            restore_role_trashed,
            find_dashboard,
            find_dashboard_by_card_number,
            find_month_balance_by_card_card,
            find_month_balance_card,
            find_month_topup_amount_by_card_card,
            find_month_topup_amount_card,
            find_month_transaction_amount_by_card_card,
            find_month_transfer_receiver_amount_by_card_card,
            find_month_transfer_receiver_amount_card,
            find_month_transfer_sender_amount_card,
            find_month_withdraw_amount_by_card_card,
            find_month_withdraw_amount_card,
            find_year_balance_by_card_card,
            find_year_balance_card,
            find_year_topup_amount_by_card_card,
            find_year_transfer_receiver_amount_by_card_card,
            find_year_transfer_sender_amount_by_card_card,
            findyear_topup_amount_card,
            findyear_transaction_amount_by_card_card,
            findyear_transaction_amount_card,
            findyear_transfer_receiver_amount_card,
            findyear_transfer_sender_amount_card,
            findyear_withdraw_amount_by_card_card,
            findyear_withdraw_amount_card,
            find_month_transaction_amount_card,
            find_month_transfer_sender_amount_by_card_card,
            find_all_card,
            find_by_id_card,
            find_by_user,
            find_by_card_number,
            find_active_card,
            create_card,
            update_card,
            trashed_card,
            delete_permanent_all_cards,
            delete_permanent_card,
            find_all_cards_trashed,
            restore_all_cards_trashed,
            restore_card_trashed,
            find_month_amount,
            find_all_transactions_by_merchant,
            find_all_transactions_by_api_key,
            find_month_amount_by_merchant,
            find_month_status_success_withdraw_by_card_number,
            find_year_status_success_withdraw_by_card_number,
            find_month_status_failed_withdraw_by_card_number,
            find_year_status_failed_withdraw_by_card_number,
            find_month_status_success_transfer_by_card_number,
            find_year_status_success_transfer_by_card_number,
            find_month_status_failed_transfer_by_card_number,
            find_year_status_failed_transfer_by_card_number,
            find_month_status_success_transaction_by_card_number,
            find_year_status_success_transaction_by_card_number,
            find_month_status_failed_transaction_by_card_number,
            find_year_status_failed_transaction_by_card_number,
            find_month_status_success_by_card_number,
            find_year_status_success_by_card_number,
            find_month_status_failed_by_card_number,
            find_year_status_failed_by_card_number,
            find_month_payment_method,
            find_month_payment_method_by_merchant,
            find_month_total_amount,
            find_month_total_amount_by_merchant,
            find_year_amount,
            find_year_amount_by_merchant,
            find_year_payment_method,
            find_year_payment_method_by_merchant,
            find_year_total_amount,
            find_year_total_amount_by_merchant,
            find_all_merchants,
            find_all_transactions,
            find_by_id,
            find_by_api_key,
            create_merchant,
            update_merchant,
            trashed_merchant,
            delete_permanent_all_merchant,
            delete_permanent_merchant,
            find_all_merchants_trashed,
            restore_merchant_all_trashed,
            restore_merchant_trashed,
            find_month_total_balance,
            find_year_total_balance,
            find_month_balance,
            find_year_balance,
            find_all_saldos,
            find_by_id_saldo,
            find_by_active_saldo,
            create_saldo,
            update_saldo,
            trashed_saldo,
            find_all_saldos_trashed,
            restore_saldo_trashed,
            delete_permanent_saldo,
            restore_saldo_all_trashed,
            delete_permanent_all_saldo,
            create_topup,
            find_all_topups,
            find_by_active_topup,
            find_by_card_number_topup,
            find_by_id_topup,
            find_month_status_failed,
            find_month_status_success,
            find_month_topup_amount,
            find_month_topup_method,
            find_month_topup_method_by_card,
            find_year_status_failed,
            find_year_status_success,
            find_year_topup_amount,
            find_year_topup_method,
            find_year_topup_method_by_card,
            find_month_topup_method_by_card,
            find_year_topup_method_by_card,
            trashed_topup,
            update_topup,
            delete_permanent_all_topup,
            delete_permanent_topup,
            find_all_topups_trashed,
            restore_topup_all_trashed,
            restore_topup_trashed,
            find_all_transactions_transaction,
            find_by_card_number_transaction,
            find_by_id_transaction,
            find_month_status_failed_transaction,
            find_month_status_success_transaction,
            find_month_transaction_amount,
            find_month_transaction_amount_by_card,
            find_month_transaction_method,
            find_month_transaction_method_by_card,
            find_year_status_failed_transaction,
            find_year_status_success_transaction,
            find_year_transaction_amount,
            find_year_transaction_amount_by_card,
            find_year_transaction_method,
            find_year_transaction_method_by_card,
            find_by_merchant_transaction,
            find_by_active_transaction,
            find_by_merchant_user,
            find_by_active_merchant,
            find_month_topup_amount_by_card,
            find_year_topup_amount_by_card,
            create_transaction,
            update_transaction,
            trashed_transaction,
            find_all_transactions_trashed,
            restore_transaction_trashed,
            delete_permanent_transaction,
            restore_transaction_all_trashed,
            delete_permanent_all_transaction,
            create_transfer,
            find_all_transfers,
            find_by_active_transfer,
            find_by_id_transfer,
            find_by_transfer_from,
            find_by_transfer_to,
            find_month_status_failed_transfer,
            find_month_status_success_transfer,
            find_month_transfer_amount_by_receiver,
            find_month_transfer_amount_by_sender,
            find_month_transfer_amount_transfer,
            find_year_status_failed_transfer,
            find_year_status_success_transfer,
            find_year_transfer_amount_by_receiver,
            find_year_transfer_amount_by_sender,
            find_year_transfer_amount_transfer,
            trashed_transfer,
            update_transfer,
            delete_permanent_all_transfer,
            delete_permanent_transfer,
            find_all_transfers_trashed,
            restore_transfer_all_trashed,
            restore_transfer_trashed,
            create_withdraw,
            find_all_withdraws,
            find_by_active_withdraw,
            find_by_card_number_withdraw,
            find_by_id_withdraw,
            find_month_status_failed_withdraw,
            find_month_status_success_withdraw,
            find_month_withdraw_amount_by_card_withdraw,
            find_month_withdraw_amount_withdraw,
            find_year_status_failed_withdraw,
            find_year_status_success_withdraw,
            find_year_withdraw_amount,
            find_year_withdraw_amount_by_card,
            trashed_withdraw,
            update_withdraw,
            delete_permanent_all_withdraw,
            delete_permanent_withdraw,
            find_all_withdraws_trashed,
            restore_withdraw_all_trashed,
            restore_withdraw_trashed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
