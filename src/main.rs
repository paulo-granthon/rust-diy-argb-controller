#![no_std]
#![no_main]

mod button;
mod effects;
mod menu;
mod state;
mod r#static;
mod timer;

use crate::button::Button;
use crate::effects::repeating_rgbycm;
use crate::timer::{PressTimer, StrictPressTimer};

use core::fmt::Write as FmtWrite;
use heapless::String;
use smart_leds::SmartLedsWrite;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use menu::MENU;
use state::STATE;
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

    // const BRIGHTNESS_STEP: u8 = 32;

    const POLL_MS: u32 = 20;
    const BUTTON_HOLD_INTERVAL_MS: u32 = 200;

    let mut button_a = Button::new(
        PressTimer::new(BUTTON_HOLD_INTERVAL_MS),
        pins.d4.into_pull_up_input(),
        || MENU.with(|m| m.next()),
    );

    let mut button_b = Button::new(
        StrictPressTimer::new(BUTTON_HOLD_INTERVAL_MS),
        pins.d5.into_pull_up_input(),
        || (MENU.with(|m| m.current_item().action)()),
    );

    // Text style
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    loop {
        button_a.update(POLL_MS);
        button_b.update(POLL_MS);

        let current_brightness = STATE.with(|s| s.brightness.get());
        let current_phase_offset = STATE.with(|s| s.phase_offset.get());

        let leds = repeating_rgbycm::<NUM_LEDS>(current_phase_offset);

        ws.write(smart_leds::brightness(
            leds.iter().cloned(),
            current_brightness,
        ))
        .unwrap();

        // Prepare tiny heapless Strings for formatting (no heap)
        let mut left: String<32> = String::new();
        let mut right: String<32> = String::new();
        let mut bottom: String<32> = String::new();

        // Format: "O: <cur> / <max>"
        write!(left, "O: {} / {}", current_phase_offset + 1, 8u8).ok();

        // Format: "B: <cur> / <max>"
        write!(right, "B: {} / {}", current_brightness, u8::MAX).ok();

        // Format: "<menu item name>"
        MENU.with(|m| write!(bottom, "{}", m.current_item().name).ok());

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

        // Draw bottom row at x=0, y=22 (bottom of 32px display minus font height)
        Text::with_baseline(&bottom, Point::new(0, 22), text_style, Baseline::Top)
            .draw(&mut disp)
            .ok();

        // Push buffer to the display
        disp.flush().ok();

        arduino_hal::delay_ms(POLL_MS);
    }
}
