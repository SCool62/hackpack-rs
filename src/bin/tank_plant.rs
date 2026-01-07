#![no_std]
#![no_main]

use arduino_hal::{
    delay_ms,
    hal::simple_pwm,
    prelude::*,
    simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer1Pwm, Timer2Pwm},
};
use hackpack::actuator::drv8835::{Drv8835, MotorState};

use core::{panic::PanicInfo, u8};

use arduino_hal::default_serial;

use embedded_hal::pwm::SetDutyCycle;

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

    // Setup bumper inputs
    // Could be interrupts but i dont wanna
    let left_bumper = pins.d3.into_pull_up_input();
    let right_bumper = pins.d2.into_pull_up_input();

    // TODO: PRESCALER DIRECT??
    let timer0pwm = Timer0Pwm::new(peripherals.TC0, Prescaler::Direct);
    
    let mut d6 = pins.d6.into_output().into_pwm(&timer0pwm);
    let mut d5 = pins.d5.into_output().into_pwm(&timer0pwm);
    // Need to enable PWM first before using
    d6.enable();
    d5.enable();

    let mut motor_driver = 
        Drv8835::new(
            d6,
            pins.d7.into_output(), 
            d5, 
            pins.d4.into_output()
        );

    
    motor_driver.drive_left_motor(MotorState::Forward(255)).unwrap();
    motor_driver.drive_right_motor(MotorState::Backward(255)).unwrap();

    loop {
        
    }
}
