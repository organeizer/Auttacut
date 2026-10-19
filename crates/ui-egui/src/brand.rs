//! The Auttacut wordmark, set in the UI typeface. (The ArtCraft marks FilmCraft ships are not
//! used: forks may not carry them, see upstream `docs/brand/LICENSE-brand.txt`.)

/// The wordmark text.
pub const WORDMARK: &str = "Auttacut";

/// Draw the wordmark `height` points tall with its left edge at `left_center` (vertically centred),
/// in light ink on dark themes (`dark = true`). Returns the drawn rect.
pub fn paint_wordmark(ui: &egui::Ui, left_center: egui::Pos2, height: f32, dark: bool) -> Option<egui::Rect> {
    let color = if dark { egui::Color32::WHITE } else { egui::Color32::from_gray(20) };
    let r = ui.painter().text(left_center, egui::Align2::LEFT_CENTER, WORDMARK, crate::theme::Tokens::semibold(height * 1.2), color);
    Some(r)
}
