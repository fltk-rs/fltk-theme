use crate::{cmap, ColorMap};
use fltk::utils::oncelock::Lazy;

const fn gray_ramp(light: u8, dark: u8, n: u8) -> u8 {
    let l = light as u16;
    let d = dark  as u16;
    (d + ((l - d) * n as u16 + 11) / 23) as u8
}

const fn gray_ramp_inv(light: u8, dark: u8, n: u8) -> u8 {
    let l = light as u16;
    let d = dark  as u16;
    (l - ((l - d) * n as u16 + 11) / 23) as u8
}

const fn cube_chan(i: u8, steps: u8, max: u8) -> u8 {
    ((i as u16 * max as u16 + ((steps - 1) / 2) as u16)
        / (steps as u16 - 1)) as u8
}

pub fn make_dark_theme(
    overrides: &'static [(u8, u8, u8, u8)],
    ramp: (u8, u8),
    cube_max: u8,
) -> Lazy<[ColorMap; 256]> {
    let (ramp_light, ramp_dark) = ramp;

    Lazy::new(move || {
        std::array::from_fn(|idx| {
            let idx8 = idx as u8;

            if let Some(&(_, r, g, b)) =
                overrides.iter().rev().find(|&&(i, ..)| i == idx8)
            {
                return cmap!(idx8, r, g, b);
            }

            if (32..=55).contains(&idx) {
                let shade = gray_ramp_inv(ramp_light, ramp_dark, idx8 - 32);
                return cmap!(idx8, shade, shade, shade);
            }

            if idx >= 56 {
                let n  = idx8 - 56;
                let b  = n / (5 * 8);
                let r  = (n / 8) % 5;
                let g  =  n      % 8;
                let rr = cube_chan(r, 5, cube_max);
                let gg = cube_chan(g, 8, cube_max);
                let bb = cube_chan(b, 5, cube_max);
                return cmap!(idx8, rr, gg, bb);
            }

            cmap!(idx8, 0, 0, 0)
        })
    })
}

pub fn make_light_theme(
    overrides: &'static [(u8, u8, u8, u8)],
    ramp: (u8, u8),
    cube_max: u8,
) -> once_cell::sync::Lazy<[ColorMap; 256]> {
    let (ramp_dark, ramp_light) = ramp;

    once_cell::sync::Lazy::new(move || {
        std::array::from_fn(|idx| {
            let idx8 = idx as u8;

            if let Some(&(_, r, g, b)) =
                overrides.iter().rev().find(|&&(i, ..)| i == idx8)
            {
                return cmap!(idx8, r, g, b);
            }

            if (32..=55).contains(&idx) {
                let shade = gray_ramp(ramp_dark, ramp_light, idx8 - 32);
                return cmap!(idx8, shade, shade, shade);
            }

            if idx >= 56 {
                let n  = idx8 - 56;
                let b  = n / (5 * 8);
                let r  = (n / 8) % 5;
                let g  =  n      % 8;
                let rr = cube_chan(r, 5, cube_max);
                let gg = cube_chan(g, 8, cube_max);
                let bb = cube_chan(b, 5, cube_max);
                return cmap!(idx8, rr, gg, bb);
            }

            cmap!(idx8, 0, 0, 0)
        })
    })
}

pub const GRUVBOX_DARK: Lazy<[crate::ColorMap; 256]> =  make_dark_theme(
    &[
        (0, 235, 219, 178),
        (1, 251,  73,  52),
        (2, 184, 187,  38),
        (3, 250, 189,  47),
        (4, 131, 165, 152),
        (5, 211, 134, 155),
        (6, 142, 192, 124),
        (7,  60,  60,  60),
        (15, 131, 165, 152),
        (49,  40,  40,  40),
    ],
    (180, 40),   // gray ramp light → dark
    110,  
);

pub const MONOKAI_DARK: Lazy<[ColorMap; 256]> = make_dark_theme(
    &[
        ( 0, 249,249,249),  // fg
        ( 1, 249, 38,114),  // red
        ( 2, 166,226, 46),  // green
        ( 3, 253,151, 31),  // yellow/orange
        ( 4, 102,217,239),  // cyan
        ( 5, 174,129,255),  // magenta
        ( 7,  51, 52, 46),  // darkest bg
        (49,  39, 40, 34),  // FL_GRAY
    ],
    (0xCC, 0x27),   // light ≈ #CCCBB3, dark ≈ #272822
    110,
);

pub const SOLARIZED_LIGHT: Lazy<[ColorMap; 256]> = make_light_theme(
    &[
        (0 ,  42,  42,  42),  // label text (base00)
        (7 , 253, 246, 227),  // main bg   (base3)
        (49, 238, 232, 213),  // FL_GRAY   (base2)
        (1 , 220,  50,  47),  // error red
        (2 , 133, 153,   0),  // ok green
        (3 , 181, 137,   0),  // warning yellow
        (4 ,  38, 139, 210),  // info blue
        (5 , 211,  54, 130),  // magenta
        (15,  38, 139, 210),  // check-mark colour
    ],
    (200, 255),   // gray ramp: 32 = #C8C8C8, 55 = #FFFFFF
    180,          // cube max: keep shadows mildly muted
);