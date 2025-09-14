use smart_leds::RGB8;

pub fn repeating_rgbycm<const N: usize>(offset: u8) -> [RGB8; N] {
    const COLORS: [RGB8; 6] = [
        RGB8::new(255, 0, 0),   // Red
        RGB8::new(0, 255, 0),   // Green
        RGB8::new(0, 0, 255),   // Blue
        RGB8::new(255, 255, 0), // Yellow
        RGB8::new(0, 255, 255), // Cyan
        RGB8::new(255, 0, 255), // Magenta
    ];

    let v: &mut [RGB8; N] = &mut [RGB8 { r: 0, g: 0, b: 0 }; N];
    for i in 0..N {
        // modulo picks repeating color
        let c = COLORS[(i + offset as usize) % COLORS.len()];
        v[i] = c;
    }
    *v
}
