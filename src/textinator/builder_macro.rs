//! Boilerplate-less command creation, using the [`textinator_command`] macro

#![macro_use]

#[macro_export]
macro_rules! textinator_command {
    ($type_name:tt, $conversion_func:expr) => {
        use arboard::Clipboard;
        use clap::Args;
        use std::path::PathBuf;
        use text_converter::TextConverter;

        #[derive(Debug, Args)]
        pub struct $type_name {
            /// Text to convert
            #[arg(conflicts_with = "from_clipboard")]
            string_to_convert: Option<String>,
            /// Convert text from a file
            #[arg(short = 'f', long = "file", conflicts_with = "string_to_convert")]
            from_file: Option<PathBuf>,
            /// Convert text in clipboard
            #[arg(short = 'c', long = "clipboard", conflicts_with = "from_file")]
            from_clipboard: bool,
            /// Paste conversion to clipboard
            #[arg(short, long)]
            paste: bool,
        }

        impl TextConverter for $type_name {
            fn converter(_input: impl AsRef<str>) -> String {
                $conversion_func
            }
        }

        impl $type_name {
            #[allow(unused)]
            pub fn parse_args(&self) -> String {
                let mut _output = String::new();

                if let Some(ref input) = self.string_to_convert {
                    _output = Self::new_from_text(input);
                }

                if let Some(ref file) = self.from_file {
                    _output = Self::new_from_file(file);
                }

                if self.from_clipboard {
                    _output = Self::new_from_clipboard();
                }

                if self.paste {
                    Clipboard::new()
                        .expect("Error while fetching clipboard")
                        .set_text(&_output)
                        .expect("Error while pasting to clipboard");
                }
                _output
            }
        }
    };
}

#[cfg(test)]
mod tests {
    // Takes any input, and outputs "LMAO"
    textinator_command! {
        LmaoCommand, 
        "LMAO".into()
    }

    #[test]
    fn create_command_macro() {
        let lmao_ouput = LmaoCommand::converter("text that'll be replaced with LMAO");
        assert_eq!(lmao_ouput, String::from("LMAO"));
    }
}
