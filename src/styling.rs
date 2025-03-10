use eframe::egui::{Color32, FontId, Stroke, TextFormat};

static mut FONT_SIZE_HEX_VIEW: f32 = 14.;

pub fn get_current_font_size_hex_view() -> FontId {
    FontId::monospace(unsafe { FONT_SIZE_HEX_VIEW })
}

pub fn create_text_format(underline: bool, color: Color32) -> TextFormat {
    if underline {
        TextFormat {
            underline: Stroke::new(0.5, Color32::LIGHT_GRAY),
            ..TextFormat::simple(get_current_font_size_hex_view(), color)
        }
    } else {
        TextFormat::simple(get_current_font_size_hex_view(), color)
    }
}

pub static mut DISPLAY_OFFSET_IN_HEX: bool = false;
pub fn create_text_offset_format(offset: usize) -> String {
    if unsafe { DISPLAY_OFFSET_IN_HEX } {
        format!("0x{offset:04X}")
    } else {
        format!("{offset:>4}")
    }
}
