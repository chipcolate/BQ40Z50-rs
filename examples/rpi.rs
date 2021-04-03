extern crate bq40z50;

use bq40z50::BQ40Z50;
use rppal::i2c::I2c;

fn main() {
  println!("Initializing BQ40Z50");

  let dev = I2c::new().unwrap();
  let mut bms = BQ40Z50::new(dev).unwrap();

  loop {
    let temp = bms.get_temperature().unwrap();
    let volt = bms.get_voltage().unwrap();
    let curr = bms.get_current().unwrap();
    println!(
      "Temperature: {:.2}\n Voltage: {:.2}\n Current: {:.2}",
      temp, volt, curr
    );
  }
}
