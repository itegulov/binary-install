use futures::task::{Spawn, SpawnExt};
use futures::Future;
use std::io;
use std::path::Path;

pub(crate) async fn remove_dir_all(path: impl AsRef<Path>, spawner: &impl Spawn) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(async move { std::fs::remove_dir_all(path) }, spawner).await
}

pub(crate) async fn create_dir_all(path: impl AsRef<Path>, spawner: &impl Spawn) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(async move { std::fs::create_dir_all(path) }, spawner).await
}

pub(crate) async fn rename(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    spawner: &impl Spawn,
) -> io::Result<()> {
    let from = from.as_ref().to_owned();
    let to = to.as_ref().to_owned();

    asyncify(async move { std::fs::rename(from, to) }, spawner).await
}

async fn asyncify<F, T>(f: F, spawner: &impl Spawn) -> io::Result<T>
where
    F: Future<Output = io::Result<T>> + Send + 'static,
    T: Send + 'static,
{
    spawner
        .spawn_with_handle(f)
        .expect("failed to spawn a task")
        .await
}
