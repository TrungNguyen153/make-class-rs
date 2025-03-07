#![feature(let_chains)]
use eframe::egui::{FontData, FontDefinitions, FontFamily, Key, Modifiers, Vec2, Visuals};

use self::{
    app::MakeClassApp,
    global_state::{GlobalState, global_state, set_global_state, unset_global_state},
    hotkeys::HotkeyManager,
    project::ProjectData,
};

#[macro_use]
extern crate obfstr;
#[macro_use]
extern crate tracing;

mod address;
pub mod address_parser;
mod app;
pub mod class;
pub mod field;
pub mod generator;
mod global_state;
mod hotkeys;
mod inspection;
pub mod memory;
mod project;
mod styling;
mod ui;
mod utils;
pub mod value;

pub fn run_app() {
    let sub = tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_ansi(true)
        .with_level(true)
        .with_max_level(tracing::Level::DEBUG)
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ENTER)
        .with_file(false)
        .with_target(false)
        .without_time()
        .finish();
    tracing::subscriber::set_global_default(sub).unwrap();

    info!("Starting app...");
    let r = eframe::run_native(
        obfstr!("MakeClass"),
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            // load fonts
            let mut hotkeys = HotkeyManager::default();
            hotkeys.register(
                "open_last_project",
                Key::O,
                Modifiers::ALT | Modifiers::CTRL,
            );

            // load global
            set_global_state(GlobalState {
                hotkeys,
                class_list: ProjectData::load().to_class_list(),
                ..Default::default()
            });

            cc.egui_ctx.style_mut(|s| {
                s.spacing.item_spacing = Vec2::new(4., 4.);
                s.visuals = Visuals::dark();
            });

            cc.egui_ctx.set_pixels_per_point(1.);

            let mut fonts = FontDefinitions::default();
            fonts.font_data.insert(
                "roboto-mono".into(),
                FontData::from_static(include_bytes!("../fonts/RobotoMono-Regular.ttf")).into(),
            );
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .insert(0, "roboto-mono".into());
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(MakeClassApp::new()))
        }),
    );

    ProjectData::store(global_state().class_list.classes()).save();
    // cleanup global;
    unset_global_state();
    r.unwrap();
}
