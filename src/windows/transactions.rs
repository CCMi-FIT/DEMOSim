use std::collections::{HashSet, HashMap};
use eframe::epaint::Color32;
use egui::RichText;

use crate::model::{ActorRole, all_c_acts, all_c_facts, CAct, CFact, Impediment, Transaction, TransactionId};

pub fn impediments_ui(ui: &mut egui::Ui, transactions: &Vec<Transaction>, transaction: &mut Transaction) {
    let available_transactions: Vec<&Transaction> = transactions.iter().filter(|tr| **tr != *transaction).collect();
    let transactions_map: HashMap<TransactionId, String> = available_transactions.iter()
        .map(|tr| (tr.id.clone(), tr.t_id.clone())).collect();
    let mut to_delete = Vec::new();

    ui.vertical(|ui| {
        for (imp_index, mut impediment) in transaction.impediments.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                if ui.button(RichText::new("❌").color(Color32::RED)).clicked() {
                    to_delete.push(imp_index);
                }
                egui::ComboBox::from_id_source(format!("{}_{}_{}", transaction.id, "Impediment-CAct", imp_index))
                    .selected_text(impediment.impeded_c_act.to_string())
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        for impeded_c_act in all_c_acts() {
                            ui.selectable_value(&mut impediment.impeded_c_act, impeded_c_act.to_owned(), impeded_c_act.to_owned().to_string());
                        }
                    });
                ui.add_space(5.0);
                egui::ComboBox::from_id_source(format!("{}_{}_{}", transaction.id, imp_index, "Impeding-Transaction"))
                    .selected_text(transactions_map.get(&impediment.impeding_transaction_id).unwrap_or(&"tr not found".to_string()).to_owned())
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(20.0);
                        for transaction in available_transactions.iter() {
                            ui.selectable_value(&mut impediment.impeding_transaction_id, transaction.id.clone(), transactions_map.get(&transaction.id).unwrap_or(&"tr not found".to_string()));
                        }
                    });
                egui::ComboBox::from_id_source(format!("{}_{}_{}", transaction.id, imp_index, "Impeding-CFact"))
                    .selected_text(impediment.impeding_c_fact.to_string())
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(20.0);
                        for c_fact in all_c_facts().iter() {
                            ui.selectable_value(&mut impediment.impeding_c_fact, c_fact.clone(), c_fact.to_string());
                        }
                    });
            });
        }
        if ui.button(RichText::new("➕").color(Color32::GREEN)).clicked() {
            transaction.impediments.push(Impediment {
                impeded_c_act: CAct::default(),
                    impeding_transaction_id: available_transactions[0].id.clone(),
                    impeding_c_fact: CFact::default(),
            });
        }
        for index in to_delete.into_iter().rev() {
            transaction.impediments.remove(index);
        }
    });
}

pub fn transactions_ui(ui: &mut egui::Ui, actor_roles: &Vec<ActorRole>, transactions: &mut Vec<Transaction>) {
    let mut to_delete = Vec::new();
    egui::Grid::new("Actor Roles")
        .striped(true)
        .spacing(&[10.0, 40.0])
        .show(ui, |ui| {
            ui.strong("Action");
            ui.strong("Id");
            ui.strong("Name");
            ui.strong("Product");
            ui.strong("Initiator");
            ui.strong("Executor");
            ui.strong("Impediments");
            ui.end_row();

            let used_executors: HashSet<_> = transactions.iter().map(|tr| &tr.executor).collect();
            let available_executors: Vec<ActorRole> = actor_roles.iter()
                .filter(|aar| !used_executors.contains(aar))
                .cloned()
                .collect();

            let transactions_cloned = transactions.to_owned();
            for (t_index, mut transaction) in transactions.iter_mut().enumerate() {
                let can_delete = !transactions_cloned.iter().any(|tr| tr.impediments.iter().any(|imp| imp.impeding_transaction_id == transaction.id));
                ui.add_enabled_ui(can_delete, |ui| {
                    if ui.button(RichText::new("❌").color(Color32::RED))
                        .on_disabled_hover_text("Used in an impediment")
                        .clicked() {
                        to_delete.push(t_index);
                    }
                });
                // if ui.button(RichText::new("❌").color(Color32::RED)).clicked() {
                //     to_delete.push(t_index);
                // }
                ui.add(egui::TextEdit::singleline(&mut transaction.t_id).min_size([50.0, 20.0].into()));
                ui.add(egui::TextEdit::singleline(&mut transaction.name).min_size([200.0, 20.0].into()));
                ui.add(egui::TextEdit::singleline(&mut transaction.product).min_size([200.0, 20.0].into()));
                // Initiator actor role combo
                egui::ComboBox::from_id_source(format!("{}_{}_{}", "Initiator", transaction.id, t_index))
                    .selected_text(transaction.initiator.name.to_owned())
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        for actor_role in actor_roles {
                            ui.selectable_value(&mut transaction.initiator, actor_role.to_owned(), actor_role.to_owned().name);
                        }
                    });
                // Executor actor role combo
                egui::ComboBox::from_id_source(format!("{}_{}_{}", "Executor", transaction.id, t_index))
                    .selected_text(transaction.executor.name.to_owned())
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        for actor_role in available_executors.clone() {
                            ui.selectable_value(&mut transaction.executor, actor_role.to_owned(), actor_role.to_owned().name);
                        }
                    });
                ui.add_space(16.0);
                impediments_ui(ui, &transactions_cloned, &mut transaction);
                ui.end_row();
            }
        });
    for index in to_delete.into_iter().rev() {
        transactions.remove(index);
    }
    ui.add_space(16.0);
    if ui.button(RichText::new("➕").color(Color32::GREEN)).clicked() {
        transactions.push(Transaction::new());
    }
}