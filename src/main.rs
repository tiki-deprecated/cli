use clap::{Parser, Subcommand};
use cli_table::{print_stdout, Table, WithTitle};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use tracing_subscriber;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, hide = true, env = "TIKI_API_KEY")]
    api_key: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    GetProfile {},
    ListCleanrooms {},
    ListSubscriptions {},

    #[command(subcommand)]
    Account(AccountCommands),

    #[command(subcommand)]
    Cleanroom(CleanroomCommands),

    #[command(subcommand)]
    Query(QueryCommands),

    #[command(subcommand)]
    Subscription(SubscriptionCommands),
}

#[derive(Subcommand)]
enum AccountCommands {
    GetProfile {},
    UpdateProfile {},
}

#[derive(Subcommand)]
enum CleanroomCommands {
    Create {},
    Get {},
    List {},
}

#[derive(Subcommand)]
enum QueryCommands {
    Create {},
}

#[derive(Subcommand)]
enum SubscriptionCommands {
    Get {},
    List {},
    Purchase {},
}

fn main() {
    let _subscriber = tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::GetProfile { .. }) => {
            get_profile(cli.api_key);
        }
        Some(Commands::ListCleanrooms { .. }) => {
            list_cleanrooms(cli.api_key);
        }
        Some(Commands::ListSubscriptions { .. }) => {
            println!("Executed the list-subscriptions subcommand");
        }
        Some(Commands::Account(_)) => {
            println!("Executed the account subcommand");
        }
        Some(Commands::Cleanroom(_)) => {
            println!("Executed the cleanroom subcommand");
        }
        Some(Commands::Query(_)) => {
            println!("Executed the query subcommand");
        }
        Some(Commands::Subscription(_)) => {
            println!("Executed the subscription subcommand");
        }
        None => {}
    }
}

#[tokio::main]
async fn get_profile(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let jwt_header = jsonwebtoken::decode_header(&token);
    match jwt_header {
        Ok(_) => debug!("jwt header validation successful"),
        Err(e) => error!("jwt validation failed: {:?}", e),
    };
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
            println!("Bad token, bro");
        },
        _ => {
            error!("Something bad!");
        },
    };

    let json = response.json::<AccountProfileRsp>().await?;
    println!("Hi, {}", json.email);

    Ok(())
}

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
#[derive(Table)]
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
