mod add;

use clap::Parser;
use enum_dispatch::enum_dispatch;

pub use self::add::add;

pub use self::add::AddOpts;

type ReplResult = Result<Option<String>, reedline_repl_rs::Error>;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum ReplCommand {
    #[command(
        name = "add",
        about = "Add files to the chat so aider can edit them or review them in detail"
    )]
    Add(AddOpts),
}
