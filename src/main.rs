#![no_std]
#![no_main]

mod effects;
mod timer;

use crate::effects::repeating_rgbycm;
use crate::timer::{CustomTimer, PressTimer, StrictPressTimer};

use panic_halt as _;
use smart_leds::SmartLedsWrite;
use ws2812_spi::Ws2812;

const NUM_LEDS: usize = 5;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let (spi, _cs) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d15.into_output(),        // SCK
        pins.d14.into_output(),        // MOSI
        pins.d16.into_pull_up_input(), // MISO (not used, but needed for SPI init)
        pins.led_rx.into_output(),     // CS (not used)
        arduino_hal::spi::Settings {
            data_order: arduino_hal::spi::DataOrder::MostSignificantFirst,
            clock: arduino_hal::spi::SerialClockRate::OscfOver4,
            ..Default::default()
        },
    );

    const BRIGHTNESS_STEP: u8 = 32;

    let mut brightness: u8 = 32;
    let mut offset: u8 = 0;

    let mut ws = Ws2812::new(spi);

    let button_a = pins.d2.into_pull_up_input();
    let button_b = pins.d3.into_pull_up_input();

    const BUTTON_HOLD_INTERVAL_MS: u32 = 200;

    let mut press_timer = PressTimer::new(BUTTON_HOLD_INTERVAL_MS);
    let mut strict_timer = StrictPressTimer::new(BUTTON_HOLD_INTERVAL_MS);

    const POLL_MS: u32 = 50;
    let mut current_ms: u32 = 0;

    loop {
        if press_timer.update(button_a.is_low(), POLL_MS) {
            offset = offset.wrapping_add(1);
        }
        if strict_timer.update(button_b.is_low(), POLL_MS) {
            brightness = brightness.wrapping_sub(BRIGHTNESS_STEP);
        }

        let leds = repeating_rgbycm::<NUM_LEDS>(offset);

        ws.write(smart_leds::brightness(leds.iter().cloned(), brightness))
            .unwrap();

        arduino_hal::delay_ms(POLL_MS);
        current_ms = current_ms.wrapping_add(POLL_MS);
    }
}
