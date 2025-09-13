use smart_leds::RGB8;


pub fn repeating_rgbcym<const N: usize>() -> [RGB8; N] {
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

