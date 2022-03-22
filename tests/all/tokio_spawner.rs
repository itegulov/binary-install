use futures::future::FutureObj;
use futures::task::{Spawn, SpawnError};
use tokio::runtime::Handle;

pub struct TokioSpawner(Handle);

impl TokioSpawner {
    pub fn new(handle: Handle) -> Self {
        TokioSpawner(handle)
    }

    pub fn current() -> Self {
        TokioSpawner::new(Handle::current())
    }
}

impl Spawn for TokioSpawner {
    fn spawn_obj(&self, obj: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        self.0.spawn(obj);
        Ok(())
    }
}
