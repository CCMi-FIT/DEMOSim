use std::collections::HashSet;
use eframe::emath::Vec2;

use crate::model::{ActorRole, Transaction};

pub fn transactions_ui(ui: &mut egui::Ui, actor_roles: &Vec<ActorRole>, transactions: &mut Vec<Transaction>) {
    let mut to_delete = Vec::new();
    egui::Grid::new("Actor Roles")
        .striped(true)
        .show(ui, |ui| {
            ui.strong("Id");
            ui.strong("Name");
            ui.strong("Product");
            ui.strong("Initiator");
            ui.strong("Executor");
            // ui.strong("Impediments");
            ui.end_row();

            let used_executors: HashSet<_> = transactions.iter().map(|tr| &tr.executor).collect();
            let available_executors: Vec<ActorRole> = actor_roles.iter()
                .filter(|aar| !used_executors.contains(aar))
                .cloned()
                .collect();
            for (index, transaction) in transactions.iter_mut().enumerate() {
                ui.add(egui::TextEdit::singleline(&mut transaction.t_id).min_size(Vec2 { x: 50.0, y: 20.0 }));
                ui.add(egui::TextEdit::singleline(&mut transaction.name).min_size(Vec2 { x: 200.0, y: 20.0 }));
                ui.add(egui::TextEdit::singleline(&mut transaction.product).min_size(Vec2 { x: 200.0, y: 20.0 }));
                // Initiator actor role combo
                egui::ComboBox::from_id_source(format!("{}_{}_{}", "Initiator", transaction.id, index))
                    .selected_text(transaction.initiator.name.to_owned())
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        for actor_role in actor_roles {
                            ui.selectable_value(&mut transaction.initiator, actor_role.to_owned(), actor_role.to_owned().name);
                        }
                    });
                // Executor actor role combo
                egui::ComboBox::from_id_source(format!("{}_{}_{}", "Executor", transaction.id, index))
                    .selected_text(transaction.executor.name.to_owned())
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        for actor_role in available_executors.clone() {
                            ui.selectable_value(&mut transaction.executor, actor_role.to_owned(), actor_role.to_owned().name);
                        }
                    });

                if ui.button("Delete").clicked() {
                    to_delete.push(index);
                }
                ui.end_row();
            }
        });
    for index in to_delete.into_iter().rev() {
        transactions.remove(index);
    }
    if ui.button("Add").clicked() {
        transactions.push(Transaction::new());
    }
}