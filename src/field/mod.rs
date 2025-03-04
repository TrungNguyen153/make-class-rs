use eframe::egui;

pub type FieldId = u64;
pub trait Field {
    fn id(&self) -> FieldId;

    fn name(&self) -> Option<String>;

    fn set_name(&self, new_name: String);

    fn field_size(&self) -> usize;

    fn draw(&self, ui: &mut egui::Ui);
}
