use jsonwebtoken;
use tracing::{debug, error};

pub async fn validate_jwt(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let jwt_header = jsonwebtoken::decode_header(&token);
    match jwt_header {
        Ok(_) => debug!("jwt header validation successful"),
        Err(e) => error!("jwt validation failed: {:?}", e),
    };
}
