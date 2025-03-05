use egui_notify::Toasts;

use crate::{
    address_parser::AddressResolver,
    class::class_list::ClassList,
    hotkeys::HotkeyManager,
    memory::{MemoryReaderWriter, MemoryState, NullMemoryReader},
};

static mut GLOBAL: Option<GlobalState> = None;
pub fn set_global_state(state: GlobalState) {
    unsafe {
        let r = &mut *(&raw mut GLOBAL);

        r.replace(state);
    }
}

pub fn unset_global_state() {
    unsafe {
        (&mut *(&raw mut GLOBAL)).take();
    }
}

pub fn global_state() -> &'static mut GlobalState {
    unsafe { &mut *(&raw mut GLOBAL) }.as_mut().unwrap()
}

pub struct GlobalState {
    pub class_list: ClassList,
    pub hotkeys: HotkeyManager,

    pub memory: Box<dyn MemoryState>,
    pub toasts: Toasts,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            class_list: Default::default(),
            hotkeys: Default::default(),
            memory: Box::new(NullMemoryReader) as Box<dyn MemoryState>,
        }
    }
}
