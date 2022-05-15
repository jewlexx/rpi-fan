use rppal::{gpio::Gpio, system::DeviceInfo};

const FAN_PIN: u8 = 14;

fn main() -> anyhow::Result<()> {
    let info = DeviceInfo::new()?;
    let model = info.model();

    println!("{}", model);

    let mut pin = Gpio::new()?.get(FAN_PIN)?.into_output();

    pin.toggle();

    Ok(())
}
