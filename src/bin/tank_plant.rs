#![no_std]
#![no_main]

use arduino_hal::prelude::*;

use core::panic::PanicInfo;

use arduino_hal::default_serial;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    avr_device::interrupt::disable();

    // SAFETY: Even though main() already has access to the peripherals, no other code runs after the panic (and interrupts are disabled), so we know accessing peripherals again is safe
    let peripherals = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(peripherals);

    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Panic!").unwrap();

    let mut led = pins.d13.into_output();
    // Blink LED
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

// TODO
#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut serial = default_serial!(peripherals, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Peripherals setup complete").unwrap();

    loop {}
}
