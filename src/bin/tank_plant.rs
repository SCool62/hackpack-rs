#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::{
    delay_ms,
    simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer1Pwm},
};
use hackpack::actuator::{drv8835::{Drv8835, MotorState}, servo::{Servo, ServoConfig}};


use core::{panic::PanicInfo, pin, sync::atomic::{AtomicBool, Ordering}};

use arduino_hal::default_serial;

static LEFT_BUMPER_STATE: AtomicBool = AtomicBool::new(false);
static RIGHT_BUMPER_STATE: AtomicBool = AtomicBool::new(false);


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

    // Setup internal pullups for bumper pins
    pins.d2.into_pull_up_input();
    pins.d3.into_pull_up_input();
    // Setup INT0 for falling edge 0b00 would be low, 0b10 would be falling edge, 0b11 would be rising edge
    peripherals.EXINT.eicra().modify(|_, w| w.isc0().set(0b10));
    // Enable INT0
    peripherals.EXINT.eimsk().modify(|_, w| w.int0().set_bit());

    // Setup INT1 for falling edge see above
    peripherals.EXINT.eicra().modify(|_, w| w.isc1().set(0b10));
    // Enable INT1
    peripherals.EXINT.eimsk().modify(|_, w| w.int1().set_bit());


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

    
    // motor_driver.drive_left_motor(MotorState::Forward(255)).unwrap();
    // motor_driver.drive_right_motor(MotorState::Backward(255)).unwrap();

    let mut d9 = pins.d9.into_output().into_pwm(&Timer1Pwm::new(peripherals.TC1, Prescaler::Prescale256));
    d9.enable();

    let mut head_servo = Servo::new(d9, ServoConfig::new(156, 21, 0, 180).unwrap());

    // Safe because we're not enabling interrupts while in a critical section
    unsafe { avr_device::interrupt::enable(); }

    loop {
        // head_servo.set_angle(0).unwrap();
        // ufmt::uwriteln!(&mut serial, "Servo duty cycle: {}", head_servo.pwm.get_duty()).unwrap();
        // delay_ms(500);
        // head_servo.set_angle(90).unwrap();
        //         ufmt::uwriteln!(&mut serial, "Servo duty cycle: {}", head_servo.pwm.get_duty()).unwrap();

        // delay_ms(500);
        // head_servo.set_angle(180).unwrap();
        //         ufmt::uwriteln!(&mut serial, "Servo duty cycle: {}", head_servo.pwm.get_duty()).unwrap();

        // delay_ms(500);
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

        // Print whether the bumpers were clicked
        if LEFT_BUMPER_STATE.load(Ordering::Acquire) {
            ufmt::uwriteln!(&mut serial, "Left bumper pressed").unwrap();
            LEFT_BUMPER_STATE.store(false, Ordering::Relaxed);
        }

        if RIGHT_BUMPER_STATE.load(Ordering::Acquire) {
            ufmt::uwriteln!(&mut serial, "Right bumper pressed").unwrap();
            RIGHT_BUMPER_STATE.store(false, Ordering::Relaxed);
        }
    }
}

// Pin PD2 or d2 on arduino nano
// Left bumper
#[avr_device::interrupt(atmega328p)]
fn INT0() {
    // Note that the bumper was clicked, must be cleared in main loop
    LEFT_BUMPER_STATE.store(true, Ordering::Release);
}

// Pin PD3 or d3
// Right bumper
#[avr_device::interrupt(atmega328p)]
fn INT1() {
    // Note that the bumper was clicked, must be cleared in main loop
    RIGHT_BUMPER_STATE.store(true, Ordering::Release);
}
