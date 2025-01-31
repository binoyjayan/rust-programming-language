use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(disable_help_flag = true)]
pub struct Opts {
    /// Directory to start from (default is current directory)
    #[clap(default_value = ".")]
    pub dir: PathBuf,
    /// Short size in human readable format
    #[clap(short, long, default_value = "false")]
    pub human_readable: bool,
    /// Summarize disk usages
    #[clap(short, long, default_value = "false")]
    pub summarize: bool,
    /// Count links
    #[clap(short = 'l', long, default_value = "false")]
    pub count_links: bool,
    /// Show help information
    #[clap(long, short = None, action = clap::ArgAction::Help)]
    pub help: Option<bool>,
}
