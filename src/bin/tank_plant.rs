#![no_std]
#![no_main]

use arduino_hal::default_serial;


use panic_halt as _;

// TODO
#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut serial = default_serial!(peripherals, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Peripherals setup complete").unwrap();

    loop {

    }
}