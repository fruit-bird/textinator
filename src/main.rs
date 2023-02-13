mod subcommands;
mod texinator;

use clap::Parser;
use texinator::Textinator;

fn main() {
    let cli = Textinator::parse();
    cli.run();
}
