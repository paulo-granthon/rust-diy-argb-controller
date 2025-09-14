#![no_std]
#![no_main]

mod button;
mod effects;
mod timer;

use crate::button::Button;
use crate::effects::repeating_rgbycm;
use crate::timer::{PressTimer, StrictPressTimer};

use core::{cell::Cell, fmt::Write as FmtWrite};
use heapless::String;
use smart_leds::SmartLedsWrite;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use ws2812_spi::Ws2812;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use panic_halt as _;

const NUM_LEDS: usize = 5;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

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

    // --- I2C init ---
    // SDA -> D2, SCL -> D3
    // 400_000 is fast-mode; you can use 100_000 if you have issues
    let i2c: arduino_hal::I2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.d2.into_pull_up_input(),
        pins.d3.into_pull_up_input(),
        400_000,
    );

    // Build an I2C interface for the SSD1306
    let interface = I2CDisplayInterface::new(i2c);

    // Create the display driver instance
    // DisplaySize128x32 and no rotation
    let mut disp = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // Initialize the display
    disp.init().ok();
    disp.flush().ok();

    const BRIGHTNESS_STEP: u8 = 32;
    let brightness = Cell::new(32u8);
    let offset = Cell::new(0u8);

    const POLL_MS: u32 = 20;

    const BUTTON_HOLD_INTERVAL_MS: u32 = 200;

    let mut button_a = Button::new(
        PressTimer::new(BUTTON_HOLD_INTERVAL_MS),
        pins.d4.into_pull_up_input(),
        || offset.set(offset.get().wrapping_add(1) % 8),
    );
    let mut button_b = Button::new(
        StrictPressTimer::new(BUTTON_HOLD_INTERVAL_MS),
        pins.d5.into_pull_up_input(),
        || brightness.set(brightness.get().wrapping_sub(BRIGHTNESS_STEP)),
    );

    // Text style
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    loop {
        button_a.update(POLL_MS);
        button_b.update(POLL_MS);

        let leds = repeating_rgbycm::<NUM_LEDS>(offset.get());

        ws.write(smart_leds::brightness(
            leds.iter().cloned(),
            brightness.get(),
        ))
        .unwrap();

        // Prepare tiny heapless Strings for formatting (no heap)
        let mut left: String<32> = String::new();
        let mut right: String<32> = String::new();

        // Format: "O: <cur> / <max>"
        write!(left, "O: {} / {}", offset.get() + 1, 8u8).ok();

        // Format: "B: <cur> / <max>"
        write!(right, "B: {} / {}", brightness.get(), u8::MAX).ok();

        // Clear the display buffer
        disp.clear(BinaryColor::Off).ok();

        // Draw left column at x=0, y=0
        Text::with_baseline(&left, Point::new(0, 0), text_style, Baseline::Top)
            .draw(&mut disp)
            .ok();

        // Draw right column at x=64, y=0 (roughly the second column)
        Text::with_baseline(&right, Point::new(64, 0), text_style, Baseline::Top)
            .draw(&mut disp)
            .ok();

        // Push buffer to the display
        disp.flush().ok();

        arduino_hal::delay_ms(POLL_MS);
    }
}
