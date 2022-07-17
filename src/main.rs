use colored::Colorize;
use diako::cli::clap::DiakoArgs;
use clap::Parser;



fn main() {
    println!("");
    println!("{}",format!("█▀▄ █ ▄▀█ █▄▀ █▀█").purple());
    println!("{}",format!("█▄▀ █ █▀█ █░█ █▄█").purple());
    println!("------------------");
    let _diako_args =DiakoArgs::parse();

    // println!("{}",client.printpos())    
    // println!(
    //     "{}, {}, {}, {}, {}, {}, and some normal text.",
    //     format!("Bold").bold(),
    //     format!("Red").blue(),
    //     format!("Yellow").yellow(),
    //     format!("Green Strikethrough").green().strikethrough(),
    //     format!("Blue Underline").blue().underline(),
    //     format!("Purple Italics").purple().italic()
    // );
}
