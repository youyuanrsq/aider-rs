use crate::Backend;

pub struct AsyncOpenaiBackend;

impl AsyncOpenaiBackend {
    pub fn new() -> Self {
        todo!()
    }
}
impl Backend for AsyncOpenaiBackend {
    async fn add(&mut self, opts: &crate::AddOpts) -> anyhow::Result<()> {
        todo!()
    }
}
