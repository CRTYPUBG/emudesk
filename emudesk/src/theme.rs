//! EmuDesk custom dark theme.
//!
//! Dark navy/black background with electric blue (#2f6fed) and cyan
//! (#22d3ee) accent colors. Applies egui [`egui::Visuals`] and returns a
//! [`egui::Style`] the app can install.

use egui::{Color32, Visuals};

/// Primary accent blue.
pub const ACCENT_BLUE: Color32 = Color32::from_rgb(0x2f, 0x6f, 0xed);
/// Secondary accent cyan.
pub const ACCENT_CYAN: Color32 = Color32::from_rgb(0x22, 0xd3, 0xee);
/// Deep navy window background.
pub const WINDOW_FILL: Color32 = Color32::from_rgb(0x0f, 0x16, 0x22);
/// Slightly lighter panel background.
pub const PANEL_FILL: Color32 = Color32::from_rgb(0x16, 0x1e, 0x2e);
/// Default text color (light grey).
pub const TEXT_LIGHT: Color32 = Color32::from_rgb(0xcf, 0xd8, 0xe6);
/// Muted text color for secondary labels.
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x7f, 0x8c, 0xa0);

/// Build the dark egui visuals.
pub fn visuals() -> Visuals {
    let mut v = Visuals::dark();
    v.window_fill = WINDOW_FILL;
    v.panel_fill = PANEL_FILL;
    v.faint_bg_color = Color32::from_rgb(0x12, 0x1a, 0x28);
    v.extreme_bg_color = Color32::from_rgb(0x0b, 0x10, 0x1a);
    v.widgets.noninteractive.bg_fill = PANEL_FILL;
    v.widgets.inactive.bg_fill = Color32::from_rgb(0x1c, 0x26, 0x3a);
    v.widgets.hovered.bg_fill = Color32::from_rgb(0x24, 0x30, 0x48);
    v.widgets.active.bg_fill = ACCENT_BLUE;
    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, TEXT_LIGHT);
    v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, TEXT_LIGHT);
    v.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, Color32::WHITE);
    v.widgets.active.fg_stroke = egui::Stroke::new(1.5, Color32::WHITE);
    v.widgets.noninteractive.bg_stroke =
        egui::Stroke::new(1.0, Color32::from_rgb(0x2a, 0x36, 0x4e));
    v.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, Color32::from_rgb(0x2a, 0x36, 0x4e));
    v.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, ACCENT_CYAN);
    v.widgets.active.bg_stroke = egui::Stroke::new(1.0, ACCENT_BLUE);
    v.selection.bg_fill = ACCENT_BLUE;
    v.selection.stroke = egui::Stroke::new(1.0, Color32::WHITE);
    v.hyperlink_color = ACCENT_CYAN;
    v
}

/// Build the full egui style for the app.
pub fn style() -> egui::Style {
    let mut style = egui::Style {
        visuals: visuals(),
        ..Default::default()
    };
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(14.0);
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(22.0));
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(15.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(15.0));
    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(12.0));
    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visuals_are_dark() {
        let v = visuals();
        assert!(v.dark_mode);
        assert_ne!(v.window_fill, Color32::WHITE);
    }

    #[test]
    fn accents_differ_from_background() {
        assert_ne!(ACCENT_BLUE, WINDOW_FILL);
        assert_ne!(ACCENT_CYAN, WINDOW_FILL);
    }

    #[test]
    fn style_builds_ok() {
        let s = style();
        assert!(s.visuals.dark_mode);
        assert!(!s.text_styles.is_empty());
    }
}
