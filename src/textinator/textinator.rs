use crate::textinator::{mock::MockCommand, morse::MorseCommand, reverse::ReverseCommand};
use clap::{Parser, Subcommand};

/// Back in Gimmelshtump, when my father made me live as a garden gnome, I found myself modifying text inside of my head
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Textinator {
    #[clap(subcommand)]
    command: TextUtility,
}

impl Textinator {
    pub fn run(&self) {
        println!("{}", self.command.parse_utility());
    }
}

#[derive(Debug, Subcommand)]
enum TextUtility {
    /// Convert into MoCkiNg case
    Mock(MockCommand),
    /// txet esreveR
    Reverse(ReverseCommand),
    /// Convert into -- --- .-. ... . code
    Morse(MorseCommand),
}

impl TextUtility {
    fn parse_utility(&self) -> String {
        match self {
            TextUtility::Mock(m) => m.parse_args(),
            TextUtility::Reverse(r) => r.parse_args(),
            TextUtility::Morse(m) => m.parse_args(),
        }
    }
}
