use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct DiakoArgs {
    /// The subcommand to run.
    #[clap(subcommand)]
    pub diako_sub_cmd_args: DiakoSubArgs,
}

#[derive(Debug, Subcommand)]
pub enum DiakoSubArgs {
    /// To help the user configure their git client using vscode or any other editor.
    Configure(ConfigureArgs),

    /// To switch between git clients.
    Checkout(CheckoutArgs),

    /// To init the project folder with a specific git client.
    Init(InitArgs),

    /// To switch the existing remote url of the project to a new one.
    Switch(SwitchArgs),

    /// To add a new git client.
    Git(GitStructArgs),
}

// ---------- Configure ----------
#[derive(Debug, Args)]
pub struct ConfigureArgs {}

// ---------- Checkout ----------

#[derive(Debug, Args)]
pub struct CheckoutArgs {}

// ---------- Init ----------

#[derive(Debug, Args)]
pub struct InitArgs {}

// ---------- Switch ----------

#[derive(Debug, Args)]
pub struct SwitchArgs {}

// ---------- Git ----------
#[derive(Debug, Parser)]

pub struct GitStructArgs {
    /// The subcommand to run.
    #[clap(subcommand)]
    pub diako_sub_cmd_args: GitSubArgs,
}

#[derive(Debug, Parser)]
pub enum GitSubArgs {
    /// Initializes a new project with your git client.
    ///
    Init(GitInitArgs),

    /// Adds a new remote url to the project.
    AddUrl(GitAddUrlArgs),
}

#[derive(Debug, Args)]
pub struct GitInitArgs {}
#[derive(Debug, Args)]
pub struct GitAddUrlArgs {
    #[clap(long, required = false)]
    pub add_url: String,
}
