use backend::{begin_monitoring, run_server, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Config::new()?;

    let server_thread = run_server();
    let auto_thread = begin_monitoring();

    let res = server_thread.join().unwrap();
    auto_thread.await?;

    eprintln!("{}", res);

    Ok(())
}
