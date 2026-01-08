use embedded_hal::pwm::SetDutyCycle;

pub struct Servo<PWM> {
    pub pwm: PWM,
    config: ServoConfig
}

impl<PWM: SetDutyCycle> Servo<PWM> {
    pub fn new(pin: PWM, config: ServoConfig) -> Servo<PWM> {
        Servo { pwm: pin, config }
    }

    pub fn set_angle(&mut self, angle_deg: u8) -> Result<(), PWM::Error> {
        // Converts angle to duty cycle using equation from https://raspberrypi.stackexchange.com/questions/108111/what-is-the-relationship-between-angle-and-servo-motor-duty-cycle-how-do-i-impl
        // The proportion `angle_deg` is of the max angle
        let angle_proportion: f32 = (angle_deg - self.config.min_angle) as f32 / (self.config.max_angle - self.config.min_angle) as f32;
        // There's rounding problems here...
        let duty = (angle_proportion * (self.config.max_duty - self.config.min_duty) as f32) as u8 + self.config.min_duty;

        self.pwm.set_duty_cycle(duty as u16)
    }
}

pub struct ServoConfig {
    // Arduino nano works with u8s for duty cycle, although embedded hal uses u16. I will use u8 as i'm only using arduino nano
    pub(super) max_duty: u8,
    pub(super) min_duty: u8,
    pub(super) min_angle: u8,
    pub(super) max_angle: u8
}

impl ServoConfig {
    //! Creates a new servo configuration
    //! Will return None if the minmum values are greater than or equal to the maximum values
    pub fn new(max_duty: u8, min_duty: u8, min_angle: u8, max_angle: u8) -> Option<ServoConfig> {
        if (min_duty >= max_duty) || (min_angle >= max_angle) {
            return None;
        }

        Some(ServoConfig { max_duty, min_duty, min_angle, max_angle })
    }

    pub fn get_max_duty(&self) -> u8 {
        self.max_duty
    }

    pub fn get_min_duty(&self) -> u8 {
        self.min_duty
    }

    pub fn get_max_angle(&self) -> u8 {
        self.max_angle
    }

    pub fn get_min_angle(&self) -> u8 {
        self.min_angle
    }
}