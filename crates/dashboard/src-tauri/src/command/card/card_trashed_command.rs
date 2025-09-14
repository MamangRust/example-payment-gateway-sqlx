use tauri::command;

use crate::{
    domain::{
        requests::card::{findid::FindByIdCard, list::FindAllCard},
        response::card::{ApiResponseCard, ApiResponseCardAll, ApiResponsePaginationCardDeleteAt},
    },
    service::card::card_trashed_service::CardTrashedService,
};

#[command]
pub async fn find_all_cards_trashed(
    access_token: String,
    req: FindAllCard,
) -> Result<ApiResponsePaginationCardDeleteAt, String> {
    let service = CardTrashedService::new("http://localhost:5000/api".to_string());
    service
        .find_all_cards_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_card_trashed(
    access_token: String,
    req: FindByIdCard,
) -> Result<ApiResponseCard, String> {
    let service = CardTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_card_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_card(
    access_token: String,
    req: FindByIdCard,
) -> Result<ApiResponseCard, String> {
    let service = CardTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_card(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_all_cards_trashed(access_token: String) -> Result<ApiResponseCardAll, String> {
    let service = CardTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_all_cards_trashed(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_all_cards(
    access_token: String,
) -> Result<ApiResponseCardAll, String> {
    let service = CardTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_all_cards(&access_token)
        .await
        .map_err(|e| e.to_string())
}
