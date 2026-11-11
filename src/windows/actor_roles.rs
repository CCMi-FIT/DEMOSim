use crate::model::{ActorRole, Adt, Transaction};
use eframe::epaint::Color32;
use egui::RichText;

pub fn actor_roles_ui(
    ui: &mut egui::Ui,
    transactions: &[Transaction],
    actor_roles: &mut Vec<ActorRole>,
    adt: &Adt,
) {
    let mut to_delete = Vec::new();
    egui::Grid::new("Actor Roles")
        .striped(true)
        .spacing([0.0, 5.0])
        .show(ui, |ui| {
            ui.strong(" ");
            ui.strong("Name");
            ui.end_row();

            for (index, actor_role) in actor_roles.iter_mut().enumerate() {
                let can_delete = !adt.is_mapped(&actor_role.id)
                    && !transactions.iter().any(|tr| {
                        tr.initiator_id == actor_role.id || tr.executor_id == actor_role.id
                    });
                ui.add_enabled_ui(can_delete, |ui| {
                    if ui
                        .button(RichText::new("❌").color(Color32::RED))
                        .on_disabled_hover_text("Used in a transaction or the ADT")
                        .clicked()
                    {
                        to_delete.push(index);
                    }
                });
                ui.add(
                    egui::TextEdit::singleline(&mut actor_role.name).min_size([200.0, 20.0].into()),
                );
                ui.end_row();
            }
        });
    for index in to_delete.into_iter().rev() {
        actor_roles.remove(index);
    }
    ui.add_space(16.0);
    if ui
        .button(RichText::new("➕").color(Color32::GREEN))
        .clicked()
    {
        actor_roles.push(ActorRole::default());
    }
}
