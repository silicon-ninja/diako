use colored::Colorize;
use diako::cli::clap::{DiakoArgs, DiakoSubArgs};
use clap::Parser;
fn main() {
    let diako_args =DiakoArgs::parse();
    match  diako_args.diako_sub_cmd_args {
        DiakoSubArgs::Configure(configure_args) => {
            println!("{:?}", configure_args);
        },
        DiakoSubArgs::Checkout(checkout_args) => {
            println!("{:?}", checkout_args);
        },
        DiakoSubArgs::Init(init_args) => {
            println!("{:?}", init_args);
        },
        DiakoSubArgs::Switch(switch_args) => {
            println!("{:?}", switch_args);
        },
        DiakoSubArgs::Git(git_args) => {
            println!("{:?}", git_args);
        },
        _ => {
            println!("");
            println!("{}",format!("█▀▄ █ ▄▀█ █▄▀ █▀█").purple());
            println!("{}",format!("█▄▀ █ █▀█ █░█ █▄█").purple());
            println!("------------------");
        }
        
    }
    

}
