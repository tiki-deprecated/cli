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
        #[arg(short, long, required = true)]
        email: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum CleanroomCommands {
    ListCleanrooms {},
    GetCleanroom {
        #[arg(short, long, required = true)]
        cleanroom_id: Option<String>,
    },
    CreateCleanroom {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long, required = true)]
        aws_account_id: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum QueryCommands {
    CreateEstimate {
        #[arg(short, long, required = true)]
        name: Option<String>,

        #[arg(short, long)]
        query: Option<String>,

        #[arg(short, long)]
        query_file: Option<String>,

        #[arg(short, long, required = true)]
        cleanroom_id: Option<String>,
    },
    ListEstimates {},
}

#[derive(Debug, Subcommand)]
pub enum SubscriptionCommands {
    GetSubscription {
        #[arg(short, long, required = true)]
        subscription_id: Option<String>,
    },
    ListSubscriptions {
        #[arg(short, long)]
        status: Option<String>,
    },
    PurchaseSubscription {
        #[arg(short, long, required = true)]
        subscription_id: Option<String>,
    },
    PauseSubscription {
        #[arg(short, long, required = true)]
        subscription_id: Option<String>,
    },
    DeleteSubscription {
        #[arg(short, long, required = true)]
        subscription_id: Option<String>,
    },
}
