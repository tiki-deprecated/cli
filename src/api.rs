mod utils;

use jsonwebtoken;
use serde::{Deserialize, Serialize};
use reqwest;
use thiserror::{from, error};
use tracing::debug;


#[derive(Error, Debug)]
pub enum ApiError {
    #[error("API key is not a valid JWT")]
    InvalidApiKeyFormat,

    #[error("Access denied")]
    AccessDenied(#[from] reqwest::Error),

    #[error("An unknown error occured")]
    UnknownError(#[from] reqwest::Error),
}


pub async fn get_profile(token: String) -> Result<(), ApiError> {
    let _ = match jsonwebtoken::decode_header(&token) {
        Ok(_) => Ok(),
        Err(e) => return Err(ApiError::InvalidApiKeyFormat),
    };

    /* API currently returns the following:
     *
     * If bearer token is not a valid JWT:
     *
     *   status: 401
     *   response_body: |
     *     {"message":"An error occurred while attempting to decode the Jwt: Malformed token"}
     * 
     * If bearer token IS valid JWT, but with dummy values 
     *   status: 401
     *   response_body: |
     *      {"message": "An error occurred while attempting to decode the Jwt: Signed JWT rejected: Another algorithm expected, or no matching key(s) found" }
     */
    let response = reqwest::Client::new()
        .get("https://account.mytiki.com/api/latest/profile")
        .bearer_auth(token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("user validated");
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(ApiError::AccessDenied);
        },
        _ => {
            return Err(ApiError::UnknownError);
        },
    };

    let json = response.json::<AccountProfileRsp>().await?;
    println!("Hi, {}", json.email);

    Ok(())
}

/*
#[tokio::main]
async fn list_cleanrooms(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let jwt_header = jsonwebtoken::decode_header(&token);
    match jwt_header {
        Ok(_) => debug!("jwt header validation successful"),
        Err(e) => error!("jwt validation failed: {:?}", e),
    };
    let response = reqwest::Client::new()
        .get("https://account.mytiki.com/api/latest/cleanroom")
        .bearer_auth(token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("cleanrooms listed");
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Bad token, bro");
        },
        _ => {
            error!("Something bad!");
        },
    };

    let cleanrooms = response.json::<ListCleanroomsRsp>().await?;
    print_stdout(cleanrooms.cleanrooms.with_title());

    Ok(())
}
*/

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AccountProfileRsp {
    user_id: String,
    email: String,
    org_id: String,
    modified: String,
    created: String,
}

#[derive(Debug, Deserialize, Serialize)]
//#[derive(Table)]
#[serde(rename_all = "camelCase")]
struct Cleanroom {
    cleanroom_id: String,
    name: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct ListCleanroomsRsp {
    cleanrooms: Vec<Cleanroom>,
}
