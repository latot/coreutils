use clap::Parser;
use uucore::format_usage;
use uucore::translate;
use super::paths_or_stdin::PathOrStdin;

#[derive(Parser)]
#[command(
        version = uucore::crate_version!(),
        help_template = uucore::localized_help_template(uucore::util_name()),
        override_usage = format_usage(&translate!("fold-usage")),
        about = translate!("fold-about"),
        infer_long_args = true,
)]
pub struct Args {
        #[arg(
                action = clap::ArgAction::Append,
                hide = true,
                value_hint = clap::ValueHint::FilePath,
                value_parser = clap::builder::ValueParser::new(|s: &str| s.parse::<PathOrStdin>())
        )]
        pub files: Vec<PathOrStdin>,
        #[arg(
                action = clap::ArgAction::SetTrue,
                help = translate!("fold-bytes-help"),
                short = 'b',
                long = "bytes",
        )]
        pub bytes: bool,
        #[arg(
                action = clap::ArgAction::SetTrue,
                help = translate!("fold-characters-help"),
                short = 'c',
                long = "characters",
                conflicts_with = "bytes",
        )]
        pub characters: bool,
        #[arg(
                action = clap::ArgAction::SetTrue,
                help = translate!("fold-spaces-help"),
                short = 's',
                long = "spaces",
        )]
        pub spaces: bool,
        #[arg(
                help = translate!("fold-width-help"),
                short = 'w',
                long = "width",
                value_name = "WIDTH",
                allow_hyphen_values = true,
                default_value = "80",
        )]
        pub width: usize,
}

impl Args {
    pub fn custom_parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let args = handle_obsolete(args);
        Self::parse_from(args)
    }
    pub fn from_uucore_args<T>(from: T) -> Self where T: uucore::Args {
        let from: Result<Vec<_>, _> = from.map(|value| value.into_string()).collect();
        let from = from.expect("Failed to parse");
        let args = handle_obsolete(from);
        Self::parse_from(args)
    }
}


fn handle_obsolete(args: Vec<String>) -> Vec<String> {
    let mut new_params = vec![];
    let mut args: Vec<String> = args.into_iter().flat_map(|arg| {
        if arg.starts_with('-') && arg.chars().nth(1).is_some_and(|c| c.is_ascii_digit()) {
            new_params.push("-w".to_string());
            new_params.push(arg[1..].to_string());
            None
        } else {
                Some(arg)
        }
    }).collect();
    args.extend(new_params);
    args
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WidthMode {
    Columns,
    Characters,
}