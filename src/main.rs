mod cli;
mod api;
mod utils;

use clap::Parser;
use tracing::{instrument};
use tracing_subscriber;

use cli::{Cli, Commands};
use cli::{AccountCommands, CleanroomCommands, QueryCommands, SubscriptionCommands};

#[instrument]
#[tokio::main]
async fn main() {
    let _subscriber = tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Account(args) => {
            match args.command {
                AccountCommands::Login { .. } => {
                    println!("account login");
                }
                AccountCommands::GetProfile { .. } => {
                    api::get_profile(cli.api_key);
                }
                AccountCommands::UpdateProfile { .. } => {
                    println!("account update-profile");
                }
            }
        }
        Commands::Cleanroom(args) => {
            match args.command {
                CleanroomCommands::ListCleanrooms { .. } => {
                    println!("cleanroom list-cleanrooms");
                }
                CleanroomCommands::GetCleanroom { .. } => {
                    println!("cleanroom get-cleanroom");
                }
                CleanroomCommands::CreateCleanroom { .. } => {
                    println!("cleanroom create-cleanroom");
                }
            }
        }
        Commands::Query(args) => {
            match args.command {
                QueryCommands::CreateEstimate { .. } => {
                    println!("query create-estimate");
                }
                QueryCommands::ListEstimates { .. } => {
                    println!("query list-estimates");
                }
            }
        }
        Commands::Subscription(args) => {
            match args.command {
                SubscriptionCommands::GetSubscription { .. } => {
                    println!("subscription get-subscription");
                }
                SubscriptionCommands::ListSubscriptions { .. } => {
                    println!("subscription list-subscriptions");
                }
                SubscriptionCommands::PurchaseSubscription { .. } => {
                    println!("subscription purchase-subscription");
                }
                SubscriptionCommands::PauseSubscription { .. } => {
                    println!("subscription pause-subscription");
                }
                SubscriptionCommands::DeleteSubscription { .. } => {
                    println!("subscription delete-subscription");
                }
            }
        }
    }
}
