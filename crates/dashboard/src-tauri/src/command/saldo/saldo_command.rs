use tauri::command;

use crate::{
    domain::{
        requests::saldo::{
            create::CreateSaldo, findbyid::FindByIdSaldo, list::FindAllSaldo, update::UpdateSaldo,
        },
        response::saldo::{
            ApiResponseMonthSaldoBalances, ApiResponseMonthTotalSaldo, ApiResponsePaginationSaldo,
            ApiResponsePaginationSaldoDeleteAt, ApiResponseSaldo, ApiResponseYearSaldoBalances,
            ApiResponseYearTotalSaldo,
        },
    },
    service::saldo::saldo_service::SaldoService,
};

#[command]
pub async fn find_month_total_balance(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseMonthTotalSaldo, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    service
        .find_month_total_balance(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_total_balance(
    access_token: String,
    year: u32,
) -> Result<ApiResponseYearTotalSaldo, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    service
        .find_year_total_balance(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_balance(
    access_token: String,
    year: u32,
) -> Result<ApiResponseMonthSaldoBalances, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    service
        .find_month_balance(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_balance(
    access_token: String,
    year: u32,
) -> Result<ApiResponseYearSaldoBalances, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    service
        .find_year_balance(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_saldos(
    access_token: String,
    req: FindAllSaldo,
) -> Result<ApiResponsePaginationSaldo, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    match service.find_all_saldos(&access_token, req).await {
        Ok(response) => {
            // Print response dalam format JSON yang rapi
            match serde_json::to_string_pretty(&response) {
                Ok(json) => println!("{}", json),
                Err(e) => println!("Failed to serialize response: {}", e),
            }
            Ok(response)
        }
        Err(err) => {
            println!("Error fetching saldo: {}", err);
            Err(err.to_string())
        }
    }
}

#[command]
pub async fn find_by_id_saldo(
    access_token: String,
    req: FindByIdSaldo,
) -> Result<ApiResponseSaldo, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    service
        .find_by_id(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_active_saldo(
    access_token: String,
    req: FindAllSaldo,
) -> Result<ApiResponsePaginationSaldoDeleteAt, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    service
        .find_by_active(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_saldo(
    access_token: String,
    req: CreateSaldo,
) -> Result<ApiResponseSaldo, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    service
        .create_saldo(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_saldo(
    access_token: String,
    req: UpdateSaldo,
) -> Result<ApiResponseSaldo, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    service
        .update_saldo(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn trashed_saldo(
    access_token: String,
    req: FindByIdSaldo,
) -> Result<ApiResponseSaldo, String> {
    let service = SaldoService::new("http://localhost:5000/api".to_string());

    service
        .trashed_saldo(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}
