use crate::tokio_spawner::TokioSpawner;
use crate::utils;
use binary_install_async::Cache;
use std::path::Path;

#[tokio::test]
async fn it_returns_none_if_install_is_not_permitted() {
    let spawner = TokioSpawner::current();
    let binary_name = "wasm-pack";
    let binaries = vec![binary_name];

    let dir = tempfile::TempDir::new().unwrap();
    let cache = Cache::at(dir.path());

    let dl = cache
        .download(
            false,
            binary_name,
            &binaries,
            &format!("{}/{}.tar.gz", "", binary_name),
            &spawner,
        )
        .await;

    assert!(dl.is_ok());
    assert!(dl.unwrap().is_none())
}

#[tokio::test]
async fn it_downloads_tarball() {
    let spawner = TokioSpawner::current();
    let binary_name = "wasm-pack";
    let binaries = vec![binary_name];

    // Create a temporary tarball.
    let tarball = utils::create_tarball(binary_name).ok();

    // Spin up a local TcpListener.
    let server_port = utils::start_server(tarball, None).recv().unwrap();

    let url = format!("http://{}:{}", utils::TEST_SERVER_HOST, server_port);

    let dir = tempfile::TempDir::new().unwrap();
    let cache = Cache::at(dir.path());

    let dl = cache
        .download(
            true,
            binary_name,
            &binaries,
            &format!("{}/{}.tar.gz", &url, binary_name),
            &spawner,
        )
        .await;

    assert!(dl.is_ok());
    assert!(dl.unwrap().is_some())
}

#[tokio::test]
async fn it_returns_error_when_it_failed_to_download() {
    let spawner = TokioSpawner::current();
    let server_port = 7881;
    let url = format!("http://{}:{}", utils::TEST_SERVER_HOST, server_port);
    let binary_name = "wasm-pack";
    let binaries = vec![binary_name];

    let dir = tempfile::TempDir::new().unwrap();
    let cache = Cache::at(dir.path());
    let full_url = &format!("{}/{}.tar.gz", &url, binary_name);

    let dl = cache
        .download(true, binary_name, &binaries, full_url, &spawner)
        .await;

    assert!(dl.is_err());
    assert_eq!(
        &format!("failed to download from {}", full_url),
        &format!("{}", dl.unwrap_err())
    );
}

#[tokio::test]
async fn it_returns_error_when_it_failed_to_extract_tarball() {
    let spawner = TokioSpawner::current();
    let binary_name = "wasm-pack";
    let binaries = vec![binary_name];

    let dir = tempfile::TempDir::new().unwrap();
    let cache = Cache::at(dir.path());

    // Spin up a local TcpListener.
    let server_port = utils::start_server(None, None).recv().unwrap();

    let url = format!("http://{}:{}", utils::TEST_SERVER_HOST, server_port);
    let full_url = &format!("{}/{}.tar.gz", &url, binary_name);

    let dl = cache
        .download(true, binary_name, &binaries, full_url, &spawner)
        .await;

    assert!(dl.is_err());
    assert_eq!(
        &format!("failed to extract tarball from {}", full_url),
        &format!("{}", dl.unwrap_err())
    );
}

#[tokio::test]
#[should_panic(expected = "don't know how to extract http://localhost:7884/wasm-pack.bin")]
async fn it_panics_if_not_tarball_or_zip() {
    let spawner = TokioSpawner::current();
    let server_port = 7884;
    let binary_name = "wasm-pack";
    let binaries = vec![binary_name];

    let dir = tempfile::TempDir::new().unwrap();
    let cache = Cache::at(dir.path());

    // Spin up a local TcpListener.
    utils::start_server(None, Some(server_port)).recv().unwrap();

    let url = format!("http://{}:{}", utils::TEST_SERVER_HOST, server_port);
    let full_url = &format!("{}/{}.bin", &url, binary_name);

    let _ = cache
        .download(true, binary_name, &binaries, full_url, &spawner)
        .await;
}

#[test]
fn it_joins_path_with_destination() {
    let dir = tempfile::TempDir::new().unwrap();
    let cache = Cache::at(dir.path());

    assert_eq!(dir.path().join("hello"), cache.join(Path::new("hello")));
}
