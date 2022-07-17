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
    Add(AddArgs),
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


// ---------- Add ----------

#[derive(Debug, Args)]
pub struct AddArgs {}