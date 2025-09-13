#![no_std]
#![no_main]

use panic_halt as _;
use smart_leds::{SmartLedsWrite, RGB8};
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

    let mut ws = Ws2812::new(spi);
    let leds = repeating_rgbcym::<NUM_LEDS>();

    loop {
        ws.write(smart_leds::brightness(leds.iter().cloned(), 64))
            .unwrap();

        arduino_hal::delay_ms(500);
    }
}

fn repeating_rgbcym<const N: usize>() -> [RGB8; N] {
    // Define the sequence of 6 colors: R, G, B, Y, C, M
    const COLORS: [RGB8; 6] = [
        RGB8 {
            // Red
            r: 255,
            g: 0,
            b: 0,
        },
        RGB8 {
            // Green
            r: 0,
            g: 255,
            b: 0,
        },
        RGB8 {
            // Blue
            r: 0,
            g: 0,
            b: 255,
        },
        RGB8 {
            // Yellow
            r: 255,
            g: 255,
            b: 0,
        },
        RGB8 {
            // Cyan
            r: 0,
            g: 255,
            b: 255,
        },
        RGB8 {
            // Magenta
            r: 255,
            g: 0,
            b: 255,
        },
    ];

    let v: &mut [RGB8; N] = &mut [RGB8 { r: 0, g: 0, b: 0 }; N];
    for i in 0..N {
        // modulo picks repeating color
        let c = COLORS[i % COLORS.len()];
        v[i] = c;
    }
    *v
}
