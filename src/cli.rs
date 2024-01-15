use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, hide = true, env = "TIKI_API_KEY")]
    pub api_key: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Account(AccountArgs),
    Cleanroom(CleanroomArgs),
    Query(QueryArgs),
    Subscription(SubscriptionArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct AccountArgs {
    #[command(subcommand)]
    pub command: AccountCommands,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct CleanroomArgs {
    #[command(subcommand)]
    pub command: CleanroomCommands,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct QueryArgs {
    #[command(subcommand)]
    pub command: QueryCommands,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct SubscriptionArgs {
    #[command(subcommand)]
    pub command: SubscriptionCommands,
}

#[derive(Debug, Subcommand)]
pub enum AccountCommands {
    Login {},
    GetProfile {},
    UpdateProfile {
        email: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum CleanroomCommands {
    ListCleanrooms {},
    GetCleanroom {
        cleanroom_id: Option<String>,
    },
    CreateCleanroom {
        name: Option<String>,
        description: Option<String>,
        aws_account_id: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum QueryCommands {
    CreateEstimate {
        name: Option<String>,
        query: Option<String>,
        subscription_id: Option<String>,
    },
    ListEstimates {},
}

#[derive(Debug, Subcommand)]
pub enum SubscriptionCommands {
    GetSubscription {
        subscription_id: Option<String>,
    },
    ListSubscriptions {
        status: Option<String>,
    },
    PurchaseSubscription {
        subscription_id: Option<String>,
    },
    PauseSubscription {
        subscription_id: Option<String>,
    },
    DeleteSubscription {
        subscription_id: Option<String>,
    },
}
