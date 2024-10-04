#![no_std]
#![no_main]

use panic_halt as _;

const BAUD: u32 = 57600;
const GREEN_DELAY_MS: u16 = 5500;

mod traffic;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, BAUD);

    let pin_r1 = pins.d2.into_output();
    let pin_y1 = pins.d3.into_output();
    let pin_g1 = pins.d4.into_output();

    let pin_r2 = pins.d5.into_output();
    let pin_y2 = pins.d6.into_output();
    let pin_g2 = pins.d7.into_output();

    let pin_r3 = pins.d8.into_output();
    let pin_y3 = pins.d9.into_output();
    let pin_g3 = pins.d10.into_output();

    // ufmt::uwriteln!(&mut serial, "initializing traffic light 1")
    //     .expect("expected serial out to work");
    let mut t1 = traffic::TrafficLight::new(pin_r1, pin_y1, pin_g1);
    let mut t2 = traffic::TrafficLight::new(pin_r2, pin_y2, pin_g2);
    let mut t3 = traffic::TrafficLight::new(pin_r3, pin_y3, pin_g3);

    intro();
    ufmt::uwriteln!(&mut serial, "ready").expect("expected serial out to work");

    loop {
        t1.set_green();
        arduino_hal::delay_ms(GREEN_DELAY_MS);
        t1.set_red();
        arduino_hal::delay_ms(GREEN_DELAY_MS);
    }
}

#[cfg(build = "release")]
fn intro() {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        arduino_hal::usart::Baudrate::new(BAUD),
    );

    // This message will only be visible on release builds
    ufmt::uwriteln!(
        &mut serial,
        "======================== Here be project information ========================\n\
        Cebu Institute of Technology - University | Grade 10 Love | Final Project in COMPUTER X\n\
        The following students contributed to this project:\n\
        Aparte, Ivan Leonard O (Lead developer)\n\
        Bacus, Heinz Jhairo (Developer)\n\
        Bade, Josh Lordee (Marketer)\n\
        Ares, Jolianne Q (Documentation)\n\
        Nobleza, Joanne Marylee (Documentation)\n\
        Minor, Kiersten (Documentation)\n\
        Exaltacion, Mary Julianne (Construction)\n\
        Tee, Ryle Nicole (Construction)\n\
        Garcia, Armari Sebastian (Construction)\n\
        ================== This is a release build made by Ivan Aparte =================="
    )
    .expect("expected intro message to be printed");
}

#[cfg(not(build = "release"))]
fn intro() {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        arduino_hal::usart::Baudrate::new(BAUD),
    );

    ufmt::uwriteln!(
        &mut serial,
        "This is a heavily unoptimized build. Here be dragons"
    )
    .expect("expected intro message to be printed");
}
