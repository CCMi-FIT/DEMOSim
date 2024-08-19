use eframe::epaint::Color32;
use egui::{RichText, Vec2};
use crate::model::Performer;

pub fn performers_ui(ui: &mut egui::Ui, performers: &mut Vec<Performer>) {
    let mut to_delete = Vec::new();
    egui::Grid::new("Subjects")
        .striped(true)
        .spacing(&[0.0, 5.0])
        .show(ui, |ui| {
            ui.strong(" ");
            ui.strong("Name");
            ui.end_row();

            for (index, performer) in performers.iter_mut().enumerate() {
                // let can_delete = !transactions.iter().any(|tr| tr.initiator_id == performer.id || tr.executor_id == performer.id);
                let can_delete = true;
                ui.add_enabled_ui(can_delete, |ui| {
                    if ui.button(RichText::new("❌").color(Color32::RED))
                        .on_disabled_hover_text("Used in the ADT")
                        .clicked() {
                            to_delete.push(index);
                        }
                });
                ui.add(egui::TextEdit::singleline(&mut performer.name).min_size(Vec2 { x: 200.0, y: 20.0 }));
                ui.end_row();
            }
        });
    for index in to_delete.into_iter().rev() {
        performers.remove(index);
    }
    ui.add_space(16.0);
    if ui.button(RichText::new("➕").color(Color32::GREEN)).clicked() {
        performers.push(Performer::default());
    }
}
