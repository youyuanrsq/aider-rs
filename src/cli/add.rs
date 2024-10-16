use clap::{ArgMatches, Parser};

use crate::{CmdExector, ReplContext};

use super::ReplResult;

#[derive(Debug, Parser)]
pub struct AddOpts {
    #[arg(short, long, help = "file should add to the session")]
    pub files: String,
}

pub fn add(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    todo!()
}

impl CmdExector for AddOpts {
    async fn execute<T: crate::Backend>(self, backend: &mut T) -> anyhow::Result<String> {
        todo!()
    }
}
