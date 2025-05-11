use crate::{cmap, ColorMap};

const fn gray_ramp(dark: u8, light: u8, n: u8) -> u8 {
    let l = light as u16;
    let d = dark as u16;
    (d + ((l - d) * n as u16 + 11) / 23) as u8
}

const fn gray_ramp_inv(light: u8, dark: u8, n: u8) -> u8 {
    let l = light as u16;
    let d = dark as u16;
    (l - ((l - d) * n as u16 + 11) / 23) as u8
}

const fn cube_chan(i: u8, steps: u8, max: u8) -> u8 {
    ((i as u16 * max as u16 + ((steps - 1) / 2) as u16) / (steps as u16 - 1)) as u8
}

const fn make_dark_theme(
    overrides: &[(u8, u8, u8, u8)],
    ramp: (u8, u8),
    cube_max: u8,
) -> [ColorMap; 256] {
    let (light, dark) = ramp;
    let mut map = [cmap!(255, 255, 255, 255); 256];

    let mut i = 32;
    while i <= 55 {
        let v = gray_ramp_inv(light, dark, (i - 32) as u8);
        let idx8 = i as u8;
        map[i] = cmap!(idx8, v, v, v);
        i += 1;
    }
    i = 56;
    while i < 256 {
        let n = (i - 56) as u8;
        let b = n / 40;
        let r = (n / 8) % 5;
        let g = n % 8;
        let rr = cube_chan(r, 5, cube_max);
        let gg = cube_chan(g, 8, cube_max);
        let bb = cube_chan(b, 5, cube_max);
        let idx8 = i as u8;
        map[i] = cmap!(idx8, rr, gg, bb);
        i += 1;
    }

    let mut j = 0;
    while j < overrides.len() {
        let (idx, r, g, b) = overrides[j];
        map[idx as usize] = cmap!(idx, r, g, b);
        j += 1;
    }
    map
}

const fn make_light_theme(
    overrides: &'static [(u8, u8, u8, u8)],
    ramp: (u8, u8),
    cube_max: u8,
) -> [ColorMap; 256] {
    let (dark, light) = ramp;
    let mut map = [cmap!(255, 255, 255, 255); 256];

    let mut i = 32;
    while i <= 55 {
        let v = gray_ramp(dark, light, (i - 32) as u8);
        let idx8 = i as u8;
        map[i] = cmap!(idx8, v, v, v);
        i += 1;
    }
    i = 56;
    while i < 256 {
        let n = (i - 56) as u8;
        let b = n / 40;
        let r = (n / 8) % 5;
        let g = n % 8;
        let rr = cube_chan(r, 5, cube_max);
        let gg = cube_chan(g, 8, cube_max);
        let bb = cube_chan(b, 5, cube_max);
        let idx8 = i as u8;
        map[i] = cmap!(idx8, rr, gg, bb);
        i += 1;
    }

    let mut j = 0;
    while j < overrides.len() {
        let (idx, r, g, b) = overrides[j];
        map[idx as usize] = cmap!(idx, r, g, b);
        j += 1;
    }
    map
}

pub const GRUVBOX_DARK: [crate::ColorMap; 256] = make_dark_theme(
    &[
        (0, 235, 219, 178),
        (7, 60, 60, 60),
        (15, 131, 165, 152),
        (49, 40, 40, 40),
    ],
    (180, 40),
    110,
);

pub const MONOKAI: [ColorMap; 256] = make_dark_theme(
    &[
        (0, 249, 249, 249),
        (7, 51, 52, 46),
        (15, 152, 159, 177),
        (49, 39, 40, 34),
    ],
    (0xCC, 0x27),
    110,
);

pub const SOLARIZED_LIGHT: [ColorMap; 256] = make_light_theme(
    &[
        (0, 101, 123, 131),
        (7, 253, 246, 227),
        (15, 38, 139, 210),
        (49, 238, 232, 213),
    ],
    (200, 255),
    180,
);

pub const LIGHT: [ColorMap; 256] = make_light_theme(
    &[
        (0, 55, 55, 55),     // foreground
        (7, 255, 255, 255),  // background2
        (15, 0, 120, 180),   // selection
        (49, 235, 235, 235), // background
    ],
    (200, 255),
    180,
);

pub const DARK1: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 55, 55, 55),
        (7, 75, 75, 75),
        (0, 235, 235, 235),
        (15, 0, 120, 180),
    ],
    (0xCC, 0x27),
    110,
);

pub const DARK2: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 35, 35, 40),
        (7, 26, 26, 30),
        (0, 235, 235, 235),
        (15, 0, 120, 180),
    ],
    (0xCC, 0x27),
    110,
);

pub const TAN: [ColorMap; 256] = make_light_theme(
    &[
        (49, 195, 195, 181),
        (7, 243, 243, 243),
        (0, 55, 55, 55),
        (15, 0, 0, 175),
    ],
    (200, 255),
    180,
);

pub const DARK_TAN: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 165, 165, 151),
        (7, 223, 223, 223),
        (0, 55, 55, 55),
        (15, 0, 0, 175),
    ],
    (0xCC, 0x27),
    110,
);

pub const NORD: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 41, 46, 57),
        (7, 59, 66, 82),
        (0, 235, 235, 235),
        (15, 0, 120, 180),
    ],
    (0xCC, 0x27),
    110,
);

pub const MARINE: [ColorMap; 256] = make_light_theme(
    &[
        (49, 136, 192, 184),
        (7, 200, 224, 216),
        (0, 55, 55, 55),
        (15, 0, 0, 128),
    ],
    (200, 255),
    180,
);

pub const BLUEISH: [ColorMap; 256] = make_light_theme(
    &[
        (49, 210, 213, 220),
        (7, 255, 255, 255),
        (0, 55, 55, 55),
        (15, 0, 0, 128),
    ],
    (200, 255),
    180,
);

pub const HIGH_CONTRAST: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 0, 0, 0),
        (7, 20, 20, 20),
        (0, 255, 255, 255),
        (15, 0, 120, 255),
    ],
    (0xCC, 0x27),
    110,
);

pub const FOREST: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 34, 51, 34),
        (7, 46, 79, 46),
        (0, 200, 200, 200),
        (15, 64, 224, 208),
    ],
    (0xCC, 0x27),
    110,
);

pub const PURPLE_DUSK: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 48, 25, 52),
        (7, 72, 50, 72),
        (0, 220, 220, 220),
        (15, 255, 105, 180),
    ],
    (0xCC, 0x27),
    110,
);

pub const SOLARIZED_DARK: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 0, 43, 54),
        (7, 7, 54, 66),
        (0, 131, 148, 150),
        (15, 211, 54, 130),
    ],
    (0xCC, 0x27),
    110,
);

pub const GRUVBOX_LIGHT: [ColorMap; 256] = make_light_theme(
    &[
        (49, 251, 237, 193),
        (7, 235, 219, 178),
        (0, 56, 52, 46),
        (15, 69, 133, 137),
    ],
    (200, 255),
    180,
);

pub const DRACULA: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 40, 42, 54),
        (7, 68, 71, 90),
        (0, 248, 248, 242),
        (15, 189, 147, 249),
    ],
    (0xCC, 0x27),
    110,
);

pub const OCEANIC_NEXT: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 45, 52, 54),
        (7, 60, 68, 70),
        (0, 220, 220, 220),
        (15, 99, 184, 219),
    ],
    (0xCC, 0x27),
    110,
);

pub const MINIMALIST: [ColorMap; 256] = make_light_theme(
    &[
        (49, 240, 240, 240),
        (7, 230, 230, 230),
        (0, 50, 50, 50),
        (15, 100, 149, 237),
    ],
    (200, 255),
    180,
);

pub const AUTUMN: [ColorMap; 256] = make_light_theme(
    &[
        (49, 245, 245, 220),
        (7, 230, 230, 200),
        (0, 80, 50, 10),
        (15, 255, 165, 0),
    ],
    (200, 255),
    180,
);

pub const CYBERPUNK: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 30, 30, 75),
        (7, 20, 20, 50),
        (0, 0, 255, 0),
        (15, 255, 0, 255),
    ],
    (250, 30),
    180,
);

pub const MATERIAL_DARK: [ColorMap; 256] = make_dark_theme(
    &[
        (49, 28, 28, 28),
        (7, 40, 40, 40),
        (0, 255, 255, 255),
        (15, 0, 122, 255),
    ],
    (0xCC, 0x27),
    110,
);

pub const MINT: [ColorMap; 256] = make_light_theme(
    &[
        (49, 200, 240, 200),
        (7, 180, 220, 180),
        (0, 50, 100, 50),
        (15, 0, 255, 0),
    ],
    (200, 255),
    180,
);

pub const VINTAGE: [ColorMap; 256] = make_light_theme(
    &[
        (49, 240, 230, 200),
        (7, 220, 210, 180),
        (0, 80, 50, 30),
        (15, 255, 215, 0),
    ],
    (200, 255),
    180,
);

pub const GRAY: [ColorMap; 256] = make_light_theme(
    &[
        (49, 192, 192, 192),
        (7, 255, 255, 255),
        (0, 0, 0, 0),
        (15, 0, 0, 128),
    ],
    (200, 255),
    180,
);
