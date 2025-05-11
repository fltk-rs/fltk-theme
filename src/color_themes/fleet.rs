use crate::{cmap, ColorMap};
use fltk::utils::oncelock::Lazy;

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

fn make_dark_theme(
    overrides: &'static [(u8, u8, u8, u8)],
    ramp: (u8, u8),
    cube_max: u8,
) -> [ColorMap; 256] {
    let (ramp_light, ramp_dark) = ramp;

    std::array::from_fn(|idx| {
        let idx8 = idx as u8;

        if let Some(&(_, r, g, b)) = overrides.iter().rev().find(|&&(i, ..)| i == idx8) {
            return cmap!(idx8, r, g, b);
        }

        if (32..=55).contains(&idx) {
            let shade = gray_ramp_inv(ramp_light, ramp_dark, idx8 - 32);
            return cmap!(idx8, shade, shade, shade);
        }

        if idx >= 56 {
            let n = idx8 - 56;
            let b = n / (5 * 8);
            let r = (n / 8) % 5;
            let g = n % 8;
            let rr = cube_chan(r, 5, cube_max);
            let gg = cube_chan(g, 8, cube_max);
            let bb = cube_chan(b, 5, cube_max);
            return cmap!(idx8, rr, gg, bb);
        }

        cmap!(idx8, 0, 0, 0)
    })
}

fn make_light_theme(
    overrides: &'static [(u8, u8, u8, u8)],
    ramp: (u8, u8),
    cube_max: u8,
) -> [ColorMap; 256] {
    let (ramp_dark, ramp_light) = ramp;
    std::array::from_fn(|idx| {
        let idx8 = idx as u8;

        if let Some(&(_, r, g, b)) = overrides.iter().rev().find(|&&(i, ..)| i == idx8) {
            return cmap!(idx8, r, g, b);
        }

        if (32..=55).contains(&idx) {
            let shade = gray_ramp(ramp_dark, ramp_light, idx8 - 32);
            return cmap!(idx8, shade, shade, shade);
        }

        if idx >= 56 {
            let n = idx8 - 56;
            let b = n / (5 * 8);
            let r = (n / 8) % 5;
            let g = n % 8;
            let rr = cube_chan(r, 5, cube_max);
            let gg = cube_chan(g, 8, cube_max);
            let bb = cube_chan(b, 5, cube_max);
            return cmap!(idx8, rr, gg, bb);
        }

        cmap!(idx8, 0, 0, 0)
    })
}

pub static GRUVBOX_DARK: Lazy<[crate::ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (0, 235, 219, 178),
            (7, 60, 60, 60),
            (15, 131, 165, 152),
            (49, 40, 40, 40),
        ],
        (180, 40),
        110,
    )
});

pub static MONOKAI: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (0, 249, 249, 249),
            (7, 51, 52, 46),
            (15, 152, 159, 177),
            (49, 39, 40, 34),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static SOLARIZED_LIGHT: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (0, 101, 123, 131),
            (7, 253, 246, 227),
            (15, 38, 139, 210),
            (49, 238, 232, 213),
        ],
        (200, 255),
        180,
    )
});

pub static LIGHT: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (0, 55, 55, 55),     // foreground
            (7, 255, 255, 255),  // background2
            (15, 0, 120, 180),   // selection
            (49, 235, 235, 235), // background
        ],
        (200, 255),
        180,
    )
});

pub static DARK1: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 55, 55, 55),
            (7, 75, 75, 75),
            (0, 235, 235, 235),
            (15, 0, 120, 180),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static DARK2: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 35, 35, 40),
            (7, 26, 26, 30),
            (0, 235, 235, 235),
            (15, 0, 120, 180),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static TAN: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (49, 195, 195, 181),
            (7, 243, 243, 243),
            (0, 55, 55, 55),
            (15, 0, 0, 175),
        ],
        (200, 255),
        180,
    )
});

pub static DARK_TAN: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 165, 165, 151),
            (7, 223, 223, 223),
            (0, 55, 55, 55),
            (15, 0, 0, 175),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static NORD: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 41, 46, 57),
            (7, 59, 66, 82),
            (0, 235, 235, 235),
            (15, 0, 120, 180),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static MARINE: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (49, 136, 192, 184),
            (7, 200, 224, 216),
            (0, 55, 55, 55),
            (15, 0, 0, 128),
        ],
        (200, 255),
        180,
    )
});

pub static BLUEISH: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (49, 210, 213, 220),
            (7, 255, 255, 255),
            (0, 55, 55, 55),
            (15, 0, 0, 128),
        ],
        (200, 255),
        180,
    )
});

pub static HIGH_CONTRAST: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 0, 0, 0),
            (7, 20, 20, 20),
            (0, 255, 255, 255),
            (15, 0, 120, 255),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static FOREST: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 34, 51, 34),
            (7, 46, 79, 46),
            (0, 200, 200, 200),
            (15, 64, 224, 208),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static PURPLE_DUSK: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 48, 25, 52),
            (7, 72, 50, 72),
            (0, 220, 220, 220),
            (15, 255, 105, 180),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static SOLARIZED_DARK: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 0, 43, 54),
            (7, 7, 54, 66),
            (0, 131, 148, 150),
            (15, 211, 54, 130),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static GRUVBOX_LIGHT: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (49, 251, 237, 193),
            (7, 235, 219, 178),
            (0, 56, 52, 46),
            (15, 69, 133, 137),
        ],
        (200, 255),
        180,
    )
});

pub static DRACULA: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 40, 42, 54),
            (7, 68, 71, 90),
            (0, 248, 248, 242),
            (15, 189, 147, 249),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static OCEANIC_NEXT: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 45, 52, 54),
            (7, 60, 68, 70),
            (0, 220, 220, 220),
            (15, 99, 184, 219),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static MINIMALIST: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (49, 240, 240, 240),
            (7, 230, 230, 230),
            (0, 50, 50, 50),
            (15, 100, 149, 237),
        ],
        (200, 255),
        180,
    )
});

pub static AUTUMN: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (49, 245, 245, 220),
            (7, 230, 230, 200),
            (0, 80, 50, 10),
            (15, 255, 165, 0),
        ],
        (200, 255),
        180,
    )
});

pub static CYBERPUNK: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 30, 30, 75),
            (7, 20, 20, 50),
            (0, 0, 255, 0),
            (15, 255, 0, 255),
        ],
        (250, 30),
        180,
    )
});

pub static MATERIAL_DARK: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_dark_theme(
        &[
            (49, 28, 28, 28),
            (7, 40, 40, 40),
            (0, 255, 255, 255),
            (15, 0, 122, 255),
        ],
        (0xCC, 0x27),
        110,
    )
});

pub static MINT: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (49, 200, 240, 200),
            (7, 180, 220, 180),
            (0, 50, 100, 50),
            (15, 0, 255, 0),
        ],
        (200, 255),
        180,
    )
});

pub static VINTAGE: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (49, 240, 230, 200),
            (7, 220, 210, 180),
            (0, 80, 50, 30),
            (15, 255, 215, 0),
        ],
        (200, 255),
        180,
    )
});

pub static GRAY: Lazy<[ColorMap; 256]> = Lazy::new(|| {
    make_light_theme(
        &[
            (49, 192, 192, 192),
            (7, 255, 255, 255),
            (0, 0, 0, 0),
            (15, 0, 0, 128),
        ],
        (200, 255),
        180,
    )
});

/*
#pragma once
#include <array>
#include <cstdint>
#include <initializer_list>
#include <utility>

struct ColorMap {
    std::uint8_t index;
    std::uint8_t r, g, b;
};

constexpr std::uint8_t gray_ramp(std::uint8_t dark,
                                 std::uint8_t light,
                                 std::uint8_t n)              /* 0‥23 */
{

    return static_cast<std::uint8_t>(
        dark + ((light - dark) * n + 11) / 23);
}

constexpr std::uint8_t gray_ramp_inv(std::uint8_t light,
                                     std::uint8_t dark,
                                     std::uint8_t n)          /* 0‥23 */
{
    return static_cast<std::uint8_t>(
        light - ((light - dark) * n + 11) / 23);
}

constexpr std::uint8_t cube_chan(std::uint8_t i,    /* 0‥steps-1   */
                                 std::uint8_t steps,
                                 std::uint8_t max)  /* 0‥=max      */
{
    return static_cast<std::uint8_t>(
        (i * max + (steps - 1) / 2) / (steps - 1));
}

namespace detail {
inline bool lookup_override(std::uint8_t idx,
    const std::initializer_list<ColorMap>& ov,
    std::uint8_t& r, std::uint8_t& g, std::uint8_t& b)
{
    for (auto it = ov.end(); it-- != ov.begin(); ) {   // reverse scan
        if (it->index == idx) { r = it->r; g = it->g; b = it->b; return true; }
    }
    return false;
}
} // namespace detail

inline std::array<ColorMap, 256>
make_dark_theme(std::initializer_list<ColorMap> overrides,
                std::pair<std::uint8_t, std::uint8_t> ramp_light_dark,
                std::uint8_t cube_max = 110)
{
    const auto [light, dark] = ramp_light_dark;
    std::array<ColorMap, 256> cmap{};

    for (std::uint16_t idx = 0; idx < 256; ++idx) {
        std::uint8_t r = 0, g = 0, b = 0;

        if (detail::lookup_override(static_cast<std::uint8_t>(idx),
                                    overrides, r, g, b)) {
            cmap[idx] = { static_cast<std::uint8_t>(idx), r, g, b };
            continue;
        }

        if (idx >= 32 && idx <= 55) {
            std::uint8_t v = gray_ramp_inv(light, dark,
                                           static_cast<std::uint8_t>(idx - 32));
            cmap[idx] = { static_cast<std::uint8_t>(idx), v, v, v };
            continue;
        }

        if (idx >= 56) {
            std::uint8_t n = static_cast<std::uint8_t>(idx - 56);
            std::uint8_t b_ = n / (5 * 8);
            std::uint8_t r_ = (n / 8) % 5;
            std::uint8_t g_ = n % 8;
            cmap[idx] = {
                static_cast<std::uint8_t>(idx),
                cube_chan(r_, 5, cube_max),
                cube_chan(g_, 8, cube_max),
                cube_chan(b_, 5, cube_max)
            };
            continue;
        }

        cmap[idx] = { static_cast<std::uint8_t>(idx), 0, 0, 0 };
    }
    return cmap;
}

inline std::array<ColorMap, 256>
make_light_theme(std::initializer_list<ColorMap> overrides,
                 std::pair<std::uint8_t, std::uint8_t> ramp_dark_light,
                 std::uint8_t cube_max = 200)
{
    const auto [dark, light] = ramp_dark_light;
    std::array<ColorMap, 256> cmap{};

    for (std::uint16_t idx = 0; idx < 256; ++idx) {
        std::uint8_t r = 0, g = 0, b = 0;

        if (detail::lookup_override(static_cast<std::uint8_t>(idx),
                                    overrides, r, g, b)) {
            cmap[idx] = { static_cast<std::uint8_t>(idx), r, g, b };
            continue;
        }

        if (idx >= 32 && idx <= 55) {
            std::uint8_t v = gray_ramp(dark, light,
                                       static_cast<std::uint8_t>(idx - 32));
            cmap[idx] = { static_cast<std::uint8_t>(idx), v, v, v };
            continue;
        }

        if (idx >= 56) {
            std::uint8_t n = static_cast<std::uint8_t>(idx - 56);
            std::uint8_t b_ = n / (5 * 8);
            std::uint8_t r_ = (n / 8) % 5;
            std::uint8_t g_ = n % 8;
            cmap[idx] = {
                static_cast<std::uint8_t>(idx),
                cube_chan(r_, 5, cube_max),
                cube_chan(g_, 8, cube_max),
                cube_chan(b_, 5, cube_max)
            };
            continue;
        }

        cmap[idx] = { static_cast<std::uint8_t>(idx), 0, 0, 0 };
    }
    return cmap;
}

*/
