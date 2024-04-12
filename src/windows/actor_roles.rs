use egui::Vec2;
use crate::model::ActorRole;

pub fn actor_roles_ui(ui: &mut egui::Ui, actor_roles: &mut Vec<ActorRole>) {
    let mut to_delete = Vec::new();
    egui::Grid::new("Actor Roles")
        .striped(true)
        .show(ui, |ui| {
            ui.strong("Name");
            ui.end_row();

            for (index, actor_role) in actor_roles.iter_mut().enumerate() {
                ui.add(egui::TextEdit::singleline(&mut actor_role.name).min_size(Vec2 { x: 200.0, y: 20.0 }));
                if ui.button("Delete").clicked() {
                    to_delete.push(index);
                }
                ui.end_row();
            }
        });
    for index in to_delete.into_iter().rev() {
        actor_roles.remove(index);
    }
    if ui.button("Add").clicked() {
        actor_roles.push(ActorRole::default());
    }
}
