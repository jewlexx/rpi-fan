use backend::{begin_monitoring, run_server, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Config::new()?;

    let server_thread = run_server();
    let auto_thread = begin_monitoring();

    eprintln!("{}", server_thread.join().unwrap());

    if let Err(e) = auto_thread.join() {
        eprintln!("{:?}", e);
    }

    Ok(())
}
