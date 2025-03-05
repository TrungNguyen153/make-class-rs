#![feature(let_chains)]
use eframe::egui::{Key, Modifiers, Vec2};

use self::{
    app::MakeClassApp,
    global_state::{GlobalState, set_global_state, unset_global_state},
    hotkeys::HotkeyManager,
};

#[macro_use]
extern crate obfstr;
#[macro_use]
extern crate tracing;

pub mod address_parser;
mod app;
pub mod class;
pub mod field;
mod global_state;
mod hotkeys;
mod ui;

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
        Box::new(|_cc| {
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
                ..Default::default()
            });

            _cc.egui_ctx.style_mut(|s| {
                s.spacing.item_spacing = Vec2::new(8., 8.);
            });
            Ok(Box::new(MakeClassApp::new()))
        }),
    );
    // cleanup global;
    unset_global_state();
    r.unwrap();
}
