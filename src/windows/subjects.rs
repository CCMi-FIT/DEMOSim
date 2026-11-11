use crate::execution::Execution;
use crate::model::{Model, Subject};
use eframe::epaint::Color32;
use egui::{RichText, Vec2};

pub fn subjects_ui(ui: &mut egui::Ui, model: &mut Model, execution: &Execution) {
    let protected: std::collections::HashSet<_> = model
        .adt
        .mappings
        .keys()
        .map(|(_, id)| id.clone())
        .chain(
            execution
                .transactions_instances
                .iter()
                .flat_map(|i| [i.initiator_id.clone(), i.executor_id.clone()]),
        )
        .collect();
    let subjects = &mut model.subjects;
    let mut to_delete = Vec::new();
    egui::Grid::new("Subjects")
        .striped(true)
        .spacing([0.0, 5.0])
        .show(ui, |ui| {
            ui.strong(" ");
            ui.strong("Name");
            ui.end_row();

            for (index, subject) in subjects.iter_mut().enumerate() {
                // let can_delete = !transactions.iter().any(|tr| tr.initiator_id == subject.id || tr.executor_id == subject.id);
                let can_delete = !protected.contains(&subject.id);
                ui.add_enabled_ui(can_delete, |ui| {
                    if ui
                        .button(RichText::new("❌").color(Color32::RED))
                        .on_disabled_hover_text("Used in the ADT or an execution")
                        .clicked()
                    {
                        to_delete.push(index);
                    }
                });
                ui.add(
                    egui::TextEdit::singleline(&mut subject.name)
                        .min_size(Vec2 { x: 200.0, y: 20.0 }),
                );
                ui.end_row();
            }
        });
    for index in to_delete.into_iter().rev() {
        subjects.remove(index);
    }
    ui.add_space(16.0);
    if ui
        .button(RichText::new("➕").color(Color32::GREEN))
        .clicked()
    {
        subjects.push(Subject::default());
    }
}
