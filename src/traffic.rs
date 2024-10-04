use arduino_hal::delay_ms;
use embedded_hal::digital::v2::OutputPin;

const TRANSITION_DELAY_MS: u16 = 1000;

pub struct TrafficLight<RP, YP, GP> {
    red_pin: RP,
    yellow_pin: YP,
    green_pin: GP,
}

impl<RP: OutputPin, YP: OutputPin, GP: OutputPin> TrafficLight<RP, YP, GP> {
    pub fn new(mut red_pin: RP, mut yellow_pin: YP, mut green_pin: GP) -> Self {
        // red_pin.set_high();
        // delay_ms(TRANSITION_DELAY_MS);
        // red_pin.set_low();
        // yellow_pin.set_high();
        // delay_ms(TRANSITION_DELAY_MS);
        // yellow_pin.set_low();
        // green_pin.set_high();
        // delay_ms(TRANSITION_DELAY_MS);
        // green_pin.set_low();

        Self {
            red_pin,
            yellow_pin,
            green_pin,
        }
    }

    fn all_pins_off(&mut self) {
        self.red_pin.set_low();
        self.yellow_pin.set_low();
        self.green_pin.set_low();
    }

    pub fn set_red(&mut self) {
        self.all_pins_off();
        self.red_pin.set_high();
    }

    fn set_yellow(&mut self) {
        self.all_pins_off();
        self.yellow_pin.set_high();
        delay_ms(1500);
        self.yellow_pin.set_low();
    }

    pub fn set_green(&mut self) {
        self.set_yellow();
        self.green_pin.set_high();
    }
}
