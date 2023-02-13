use crate::subcommands::*;
use clap::{Parser, Subcommand};

/// A platypus?
#[derive(Debug, Parser)]
#[clap(author, version, about)]
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
    /// ConVeRt InTo MoCKING cAsE
    Mock(MockCommand),
    /// txet esreveR
    Reverse(ReverseCommand),
    /// Convert into -- --- .-. ... .
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
