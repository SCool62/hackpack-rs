#![no_std]
#![no_main]

use arduino_hal::{
    delay_ms,
    simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer1Pwm},
};
use hackpack::actuator::{drv8835::{Drv8835, MotorState}, servo::{Servo, ServoConfig}};

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

    // Setup bumper inputs
    // Could be interrupts but i dont wanna
    let _left_bumper = pins.d3.into_pull_up_input();
    let _right_bumper = pins.d2.into_pull_up_input();

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

    let mut d9 = pins.d9.into_output().into_pwm(&Timer1Pwm::new(peripherals.TC1, Prescaler::Prescale256));
    d9.enable();

    let mut head_servo = Servo::new(d9, ServoConfig::new(156, 21, 0, 180).unwrap());

    loop {
        head_servo.set_angle(0).unwrap();
        ufmt::uwriteln!(&mut serial, "Servo duty cycle: {}", head_servo.pwm.get_duty()).unwrap();
        delay_ms(500);
        head_servo.set_angle(90).unwrap();
                ufmt::uwriteln!(&mut serial, "Servo duty cycle: {}", head_servo.pwm.get_duty()).unwrap();

        delay_ms(500);
        head_servo.set_angle(180).unwrap();
                ufmt::uwriteln!(&mut serial, "Servo duty cycle: {}", head_servo.pwm.get_duty()).unwrap();

        delay_ms(500);
        // for duty in 21..=156 {
        //     d9.set_duty(duty);
        //     ufmt::uwriteln!(&mut serial, "{}", duty).unwrap();
        //     delay_ms(30);
        // }

        // d9.set_duty(25);
        //     ufmt::uwriteln!(&mut serial, "25").unwrap();
        //     delay_ms(100);
        // d9.set_duty(200);
        //     ufmt::uwriteln!(&mut serial, "200").unwrap();
        //     delay_ms(100);
    }
}
// 25, 160
// 21, 156