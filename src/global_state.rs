use crate::{class::class_list::ClassList, hotkeys::HotkeyManager};

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

#[derive(Default)]
pub struct GlobalState {
    pub class_list: ClassList,
    pub hotkeys: HotkeyManager,
}
