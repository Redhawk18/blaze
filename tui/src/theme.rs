use blaze_core::colors::{Palette, Rgb};
use cursive::{
    theme::{Color, PaletteColor, Theme},
    Cursive,
};

pub fn theme(siv: &Cursive) -> Theme {
    let mut theme = siv.current_theme().clone();

    // tui's have no concept of light and dark mode
    let palette = Palette::new(true);

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    log::info!("{:#?}", to_color(palette.secondary.default));
    theme.palette[PaletteColor::Highlight] = to_color(palette.accent.default);
    theme.palette[PaletteColor::HighlightInactive] = to_color(palette.secondary.default);
    theme.palette[PaletteColor::HighlightText] = to_color(palette.text.default);
    theme.palette[PaletteColor::Primary] = to_color(palette.text.default);
    theme.palette[PaletteColor::Secondary] = to_color(palette.text.default);
    theme.palette[PaletteColor::Shadow] = to_color(palette.secondary.default);
    theme.palette[PaletteColor::Tertiary] = to_color(palette.text.default);
    theme.palette[PaletteColor::TitlePrimary] = to_color(palette.text.default);
    theme.palette[PaletteColor::TitleSecondary] = to_color(palette.text.default);
    theme.palette[PaletteColor::View] = to_color(palette.primary.default);
    theme
}

fn to_color(rgb: Rgb) -> Color {
    Color::Rgb(
        (rgb.red * 255.0) as u8,
        (rgb.green * 255.0) as u8,
        (rgb.blue * 255.0) as u8,
    )
}