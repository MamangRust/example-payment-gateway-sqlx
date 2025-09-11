use crate::{
    abstract_trait::saldo::{
        repository::query::DynSaldoQueryRepository, service::query::SaldoQueryServiceTrait,
    },
    domain::{
        requests::saldo::FindAllSaldos,
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, SaldoResponse, SaldoResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct SaldoQueryService {
    query: DynSaldoQueryRepository,
}

impl SaldoQueryService {
    pub async fn new(query: DynSaldoQueryRepository) -> Self {
        Self { query }
    }
}

#[async_trait]
impl SaldoQueryServiceTrait for SaldoQueryService {
    async fn find_all(
        &self,
        req: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all saldos | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (saldos, total_items) = self.query.find_all(req).await.map_err(|e| {
            error!("❌ Failed to fetch all saldos: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} saldos", saldos.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let saldo_responses: Vec<SaldoResponse> =
            saldos.into_iter().map(SaldoResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Saldos retrieved successfully".to_string(),
            data: saldo_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }
    async fn find_active(
        &self,
        req: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all saldos | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (saldos, total_items) = self.query.find_active(req).await.map_err(|e| {
            error!("❌ Failed to fetch active saldos: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let saldo_responses: Vec<SaldoResponseDeleteAt> = saldos
            .into_iter()
            .map(SaldoResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Active saldos retrieved successfully".to_string(),
            data: saldo_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }
    async fn find_trashed(
        &self,
        req: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching trashed saldos | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (saldos, total_items) = self.query.find_trashed(req).await.map_err(|e| {
            error!("❌ Failed to fetch trashed saldos: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} trashed saldos", saldos.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let saldo_responses: Vec<SaldoResponseDeleteAt> = saldos
            .into_iter()
            .map(SaldoResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed saldos retrieved successfully".to_string(),
            data: saldo_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }
    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<SaldoResponse>, ServiceError> {
        info!("💳 Finding saldo by card_number={card_number}");

        let saldo = self.query.find_by_card(card_number).await.map_err(|e| {
            error!("❌ Database error while finding saldo by card_number={card_number}: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!(
            "✅ Found saldo for card_number={card_number}, id={}",
            saldo.saldo_id
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Saldo retrieved successfully".to_string(),
            data: SaldoResponse::from(saldo),
        })
    }

    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<SaldoResponse>, ServiceError> {
        info!("🔍 Finding saldo by ID: {id}");

        let saldo = self.query.find_by_id(id).await.map_err(|e| {
            error!("❌ Database error while finding saldo by ID {id}: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found saldo with ID: {id}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Saldo retrieved successfully".to_string(),
            data: SaldoResponse::from(saldo),
        })
    }
}
