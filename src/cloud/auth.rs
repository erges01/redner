use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// The payload inside our JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatorClaims {
    pub creator_id: Uuid,
    pub username: String,
    pub exp: usize, // Expiration timestamp
}

// The custom Axum Extractor!
pub struct AuthenticatedCreator(pub CreatorClaims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedCreator
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Grab the "Authorization" header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

        // 2. Make sure it's a "Bearer <token>"
        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization format".to_string()));
        }

        let token = &auth_header["Bearer ".len()..];

        // 3. Decode and validate the JWT
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key_for_dev".to_string());
        
        let token_data = decode::<CreatorClaims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        ).map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?;

        // 4. Return the authenticated creator data!
        Ok(AuthenticatedCreator(token_data.claims))
    }
}