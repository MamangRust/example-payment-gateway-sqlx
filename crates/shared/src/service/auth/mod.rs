use crate::{
    abstract_trait::{
        auth::service::AuthServiceTrait,
        hashing::DynHashing,
        jwt::DynJwtService,
        refresh_token::command::DynRefreshTokenCommandRepository,
        role::repository::query::DynRoleQueryRepository,
        token::DynTokenService,
        user::repository::{command::DynUserCommandRepository, query::DynUserQueryRepository},
        user_roles::DynUserRoleCommandRepository,
    },
    domain::{
        requests::{
            auth::{AuthRequest, RegisterRequest},
            refresh_token::UpdateRefreshToken,
            user::CreateUserRequest,
            user_role::CreateUserRoleRequest,
        },
        responses::{ApiResponse, TokenResponse, UserResponse},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct AuthService {
    query: DynUserQueryRepository,
    command: DynUserCommandRepository,
    hashing: DynHashing,
    role: DynRoleQueryRepository,
    user_role: DynUserRoleCommandRepository,
    refresh_command: DynRefreshTokenCommandRepository,
    jwt_config: DynJwtService,
    token: DynTokenService,
}

impl std::fmt::Debug for AuthService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthService")
            .field("query", &"DynUserQueryRepository")
            .field("command", &"DynUserCommandRepository")
            .field("hashing", &"Hashing")
            .field("jwt_config", &"JwtConfig")
            .field("role", &"DynJwtService")
            .field("user_role", &"DynUserRoleService")
            .field("refresh_command", &"DynRefreshTokenCommandService")
            .field("token", &"DynTokenService")
            .finish()
    }
}

pub struct AuthServiceDeps {
    pub query: DynUserQueryRepository,
    pub command: DynUserCommandRepository,
    pub hashing: DynHashing,
    pub role: DynRoleQueryRepository,
    pub user_role: DynUserRoleCommandRepository,
    pub refresh_command: DynRefreshTokenCommandRepository,
    pub jwt_config: DynJwtService,
    pub token: DynTokenService,
}

impl AuthService {
    pub async fn new(deps: AuthServiceDeps) -> Self {
        Self {
            query: deps.query,
            command: deps.command,
            hashing: deps.hashing,
            role: deps.role,
            user_role: deps.user_role,
            refresh_command: deps.refresh_command,
            jwt_config: deps.jwt_config,
            token: deps.token,
        }
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn register_user(
        &self,
        req: &RegisterRequest,
    ) -> Result<ApiResponse<UserResponse>, ServiceError> {
        info!(
            "🆕 New user registration attempt with email: {}",
            &req.email.clone(),
        );

        let existing_user = match self.query.find_by_email(req.email.clone()).await {
            Ok(user) => user,
            Err(e) => {
                error!("❌ Failed to check email in DB: {e:?}");
                return Err(ServiceError::Repo(e));
            }
        };

        if existing_user.is_some() {
            let msg = "Email already exists";
            error!("❌ [REGISTER] Email already taken | Email: {}", req.email);
            error!(msg);
            return Err(ServiceError::Custom("Email already registered".to_string()));
        }

        let hashed_password = match self.hashing.hash_password(&req.password).await {
            Ok(hash) => hash,
            Err(e) => {
                error!("❌ Failed to hash password: {e:?}");

                return Err(ServiceError::InternalServerError(
                    "Failed to hash password".into(),
                ));
            }
        };

        const DEFAULT_ROLE_NAME: &str = "ROLE_ADMIN";
        let role = match self.role.find_by_name(DEFAULT_ROLE_NAME).await {
            Ok(Some(role)) => role,
            Ok(None) => {
                error!("❌ Role not found: {}", DEFAULT_ROLE_NAME);
                return Err(ServiceError::Custom("Default role not found".to_string()));
            }
            Err(e) => {
                error!("❌ Failed to query role: {e:?}");
                return Err(ServiceError::Repo(e));
            }
        };

        let new_request = CreateUserRequest {
            firstname: req.firstname.clone(),
            lastname: req.lastname.clone(),
            password: hashed_password,
            email: req.email.clone(),
            confirm_password: req.confirm_password.clone(),
        };

        let new_user = match self.command.create(&new_request).await {
            Ok(user) => user,
            Err(e) => {
                error!("❌ Failed to create user: {e:?}");

                return Err(ServiceError::Repo(e));
            }
        };

        let assign_role_request = CreateUserRoleRequest {
            user_id: new_user.user_id,
            role_id: role.role_id,
        };

        if let Err(e) = self
            .user_role
            .assign_role_to_user(&assign_role_request)
            .await
        {
            error!(
                "❌ Failed to assign role {} to user {}: {e:?}",
                role.role_id, new_user.user_id,
            );
            return Err(ServiceError::Repo(e));
        }

        let user_response = UserResponse::from(new_user);

        info!(
            "✅ User registered successfully: {} {} ({})",
            user_response.firstname, user_response.lastname, user_response.email
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User registered successfully".to_string(),
            data: user_response,
        })
    }
    async fn login_user(
        &self,
        req: &AuthRequest,
    ) -> Result<ApiResponse<TokenResponse>, ServiceError> {
        let email = req.email.clone();

        info!("🔐 Incoming login request for user: {}", &req.email);

        let user = match self.query.find_by_email(email.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                // let msg = "User not found";
                let log_msg = format!("❌ [LOGIN] User not found | Email: {}", req.email);
                warn!("{log_msg}");

                return Err(ServiceError::Custom("user not found".to_string()));
            }
            Err(err) => {
                // let msg = format!("Error finding user: {err}");
                let log_msg = format!(
                    "🛑 [LOGIN] Database error during user lookup | Email: {} | Error: {err}",
                    req.email,
                );
                error!("{log_msg}");

                return Err(ServiceError::Repo(err));
            }
        };

        let access_token = match self.token.create_access_token(user.user_id as i32).await {
            Ok(token) => token,
            Err(e) => {
                error!("❌ Failed to generate access token: {e:?}");
                return Err(e);
            }
        };

        let refresh_token = match self.token.create_refresh_token(user.user_id as i32).await {
            Ok(token) => token,
            Err(e) => {
                error!("❌ Failed to generate refresh token: {e:?}");

                return Err(e);
            }
        };

        let token = TokenResponse {
            access_token,
            refresh_token,
        };

        info!("✅ Login successful for email: {email}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Login successful".to_string(),
            data: token,
        })
    }
    async fn get_me(&self, id: i32) -> Result<ApiResponse<UserResponse>, ServiceError> {
        info!("📄 Fetching current user profile (get_me)");

        let user = match self.query.find_by_id(id).await {
            Ok(user) => user,
            Err(e) => {
                error!("❌ Failed to fetch user from DB: {e:?}");
                return Err(ServiceError::Repo(e));
            }
        };

        let user_response = UserResponse::from(user);

        Ok(ApiResponse {
            status: "success".into(),
            message: "user fetched successfully".into(),
            data: user_response,
        })
    }

    async fn refresh_token(&self, token: &str) -> Result<ApiResponse<TokenResponse>, ServiceError> {
        info!("🔄 Refreshing access token");

        let user_id = match self.jwt_config.verify_token(token, "refresh") {
            Ok(uid) => uid,
            Err(ServiceError::TokenExpired) => {
                let _ = self.refresh_command.delete_token(token.to_string()).await;

                return Err(ServiceError::TokenExpired);
            }
            Err(e) => {
                error!("❌ Invalid token: {e:?}");
                return Err(ServiceError::InternalServerError(
                    "invalid token".to_string(),
                ));
            }
        };

        if let Err(e) = self.refresh_command.delete_token(token.to_string()).await {
            error!("❌ Failed to delete old refresh token: {e:?}");

            return Err(ServiceError::from(e));
        }

        let access_token = match self.token.create_access_token(user_id as i32).await {
            Ok(token) => token,
            Err(e) => {
                error!("❌ Failed to generate access token: {e:?}");
                return Err(e);
            }
        };

        let refresh_token = match self.token.create_refresh_token(user_id as i32).await {
            Ok(token) => token,
            Err(e) => {
                error!("❌ Failed to generate refresh token: {e:?}");
                return Err(e);
            }
        };

        let expiry = chrono::Utc::now() + chrono::Duration::hours(24);

        let update_req = &UpdateRefreshToken {
            user_id: user_id as i32,
            token: refresh_token.clone(),
            expires_at: expiry.naive_utc().to_string(),
        };

        if let Err(e) = self.refresh_command.update(update_req).await {
            error!("❌ Failed to update refresh token: {e:?}");
            return Err(ServiceError::InternalServerError(
                "Failed to update refresh token".into(),
            ));
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "token refreshed".into(),
            data: TokenResponse {
                access_token,
                refresh_token,
            },
        })
    }
}
