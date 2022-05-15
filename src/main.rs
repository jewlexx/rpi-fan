use backend::run_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_thread = run_server();

    let res = server_thread.join().unwrap();

    eprintln!("{}", res);

    Ok(())
}
