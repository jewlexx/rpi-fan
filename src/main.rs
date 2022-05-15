use rppal::{gpio::Gpio, system::DeviceInfo};

use backend::run_server;

const FAN_PIN: u8 = 14;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_thread = run_server();

    let res = server_thread.join().unwrap();

    eprintln!("{}", res);

    Ok(())
}
