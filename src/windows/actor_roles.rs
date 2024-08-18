use eframe::epaint::Color32;
use egui::{RichText, Vec2};
use crate::model::{ActorRole, Transaction};

pub fn actor_roles_ui(ui: &mut egui::Ui, transactions: &Vec<Transaction>, actor_roles: &mut Vec<ActorRole>) {
    let mut to_delete = Vec::new();
    egui::Grid::new("Actor Roles")
        .striped(true)
        .spacing(&[0.0, 5.0])
        .show(ui, |ui| {
            ui.strong(" ");
            ui.strong("Name");
            ui.end_row();

            for (index, actor_role) in actor_roles.iter_mut().enumerate() {
                let can_delete = !transactions.iter().any(|tr| tr.initiator_id == actor_role.id || tr.executor_id == actor_role.id);
                ui.add_enabled_ui(can_delete, |ui| {
                    if ui.button(RichText::new("❌").color(Color32::RED))
                        .on_disabled_hover_text("Used in a transaction")
                        .clicked() {
                            to_delete.push(index);
                        }
                });
                ui.add(egui::TextEdit::singleline(&mut actor_role.name).min_size(Vec2 { x: 200.0, y: 20.0 }));
                ui.end_row();
            }
        });
    for index in to_delete.into_iter().rev() {
        actor_roles.remove(index);
    }
    ui.add_space(16.0);
    if ui.button(RichText::new("➕").color(Color32::GREEN)).clicked() {
        actor_roles.push(ActorRole::default());
    }
}
