use eframe::epaint::Color32;
use egui::{RichText, TextWrapMode};
use std::collections::{HashMap, HashSet};
use crate::model::{all_acts, all_c_acts, all_c_facts, ActorRole, CAct, CFact, CPAct, Impediment, Initiation, Multiplicity, Transaction, TransactionId};

pub fn initiations_ui(ui: &mut egui::Ui, transactions: &Vec<Transaction>, transaction: &mut Transaction) {
    let available_transactions: Vec<&Transaction> = transactions.iter().filter(|tr| **tr != *transaction).collect();
    let transactions_map: HashMap<TransactionId, String> = available_transactions.iter()
        .map(|tr| (tr.id.clone(), tr.t_id.clone())).collect();
    let mut to_delete = Vec::new();

    ui.vertical(|ui| {
        for (in_index, initiation) in transaction.initiations.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                if ui.button(RichText::new("❌").color(Color32::RED)).clicked() {
                    to_delete.push(in_index);
                }
                egui::ComboBox::from_id_salt(format!("{}_{}_{}", transaction.id, "Initiating-CAct", in_index))
                    .selected_text(initiation.initiating_c_fact.to_string())
                    .show_ui(ui, |ui| {
                        for initiating_fact in all_c_facts() {
                            ui.selectable_value(&mut initiation.initiating_c_fact, initiating_fact.to_owned(), initiating_fact.to_string());
                        }
                    });
                ui.add_space(5.0);
                egui::ComboBox::from_id_salt(format!("{}_{}_{}", transaction.id, in_index, "Initiated-Transaction"))
                    .selected_text(transactions_map.get(&initiation.initiated_transaction_id).unwrap_or(&"tr not found".to_string()).to_owned())
                    .show_ui(ui, |ui| {
                        for transaction in available_transactions.iter() {
                            ui.selectable_value(&mut initiation.initiated_transaction_id, transaction.id.clone(), transactions_map.get(&transaction.id).unwrap_or(&"tr not found".to_string()));
                        }
                    });
                ui.add_space(5.0);
                egui::ComboBox::from_id_salt(format!("{}_{}_{}", transaction.id, in_index, "Initiated-CAct"))
                    .selected_text(initiation.initiated_c_act.to_string())
                    .show_ui(ui, |ui| {
                        for c_act in all_c_acts().iter() {
                            ui.selectable_value(&mut initiation.initiated_c_act, c_act.clone(), c_act.to_string());
                        }
                    });
                ui.add_space(5.0);
                if initiation.multiplicity_tmp_str.is_empty() {
                    initiation.multiplicity_tmp_str = initiation.multiplicity.to_string();
                }
                let multiplicity_r = initiation.multiplicity_tmp_str.parse::<Multiplicity>();
                let color = match multiplicity_r {
                    Err(_) => Some(Color32::RED),
                    Ok(_) => None,
                };
                ui.add(egui::TextEdit::singleline(&mut initiation.multiplicity_tmp_str).min_size([50.0, 20.0].into()).text_color_opt(color));
                if let Ok(multiplicity) = multiplicity_r {
                    initiation.multiplicity = multiplicity;
                }
            });
        }
        if ui.button(RichText::new("➕").color(Color32::GREEN)).clicked() {
            transaction.initiations.push(Initiation {
                initiating_c_fact: CFact::default(),
                initiated_transaction_id: available_transactions[0].id.clone(),
                initiated_c_act: CAct::default(),
                multiplicity: Multiplicity::default(),
                multiplicity_tmp_str: Multiplicity::default().to_string(),
            });
        }
        for index in to_delete.into_iter().rev() {
            transaction.initiations.remove(index);
        }
    });
}

pub fn impediments_ui(ui: &mut egui::Ui, transactions: &Vec<Transaction>, transaction: &mut Transaction) {
    let available_transactions: Vec<&Transaction> = transactions.iter().filter(|tr| **tr != *transaction).collect();
    let transactions_map: HashMap<TransactionId, String> = available_transactions.iter()
        .map(|tr| (tr.id.clone(), tr.t_id.clone())).collect();
    let mut to_delete = Vec::new();

    ui.vertical(|ui| {
        for (imp_index, impediment) in transaction.impediments.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                if ui.button(RichText::new("❌").color(Color32::RED)).clicked() {
                    to_delete.push(imp_index);
                }
                egui::ComboBox::from_id_salt(format!("{}_{}_{}", transaction.id, "Impediment-CPAct", imp_index))
                    .selected_text(impediment.impeded_act.to_string())
                    .show_ui(ui, |ui| {
                        for impeded_act in all_acts() {
                            ui.selectable_value(&mut impediment.impeded_act, impeded_act.to_owned(), impeded_act.to_string());
                        }
                    });
                ui.add_space(5.0);
                egui::ComboBox::from_id_salt(format!("{}_{}_{}", transaction.id, imp_index, "Impeding-Transaction"))
                    .selected_text(transactions_map.get(&impediment.impeding_transaction_id).unwrap_or(&"tr not found".to_string()).to_owned())
                    .show_ui(ui, |ui| {
                        for transaction in available_transactions.iter() {
                            ui.selectable_value(&mut impediment.impeding_transaction_id, transaction.id.clone(), transactions_map.get(&transaction.id).unwrap_or(&"tr not found".to_string()));
                        }
                    });
                egui::ComboBox::from_id_salt(format!("{}_{}_{}", transaction.id, imp_index, "Impeding-CFact"))
                    .selected_text(impediment.impeding_c_fact.to_string())
                    .show_ui(ui, |ui| {
                        for c_fact in all_c_facts().iter() {
                            ui.selectable_value(&mut impediment.impeding_c_fact, c_fact.clone(), c_fact.to_string());
                        }
                    });
            });
        }
        if ui.button(RichText::new("➕").color(Color32::GREEN)).clicked() {
            transaction.impediments.push(Impediment {
                impeded_act: CPAct::CAct(CAct::default()),
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
            ui.strong("Initiations\nInitiating C-Fact | Initiated Transaction | Iniciated C-Act | Multiplicity");
            ui.strong("Wait Links\nImpeded Fact | Impeding Transaction | Impeding C-Fact");
            ui.end_row();

            let used_executors: HashSet<_> = transactions.iter().map(|tr| tr.executor_id.clone()).collect();
            let available_executors: Vec<&ActorRole> = actor_roles.iter()
                .filter(|aar| !used_executors.contains(&aar.id))
                .collect();
            let transactions_cloned = transactions.to_owned();
            for (t_index, mut transaction) in transactions.iter_mut().enumerate() {
                let initiator: Option<ActorRole> = actor_roles.iter().find(|ar| ar.id == transaction.initiator_id).cloned();
                let executor: Option<ActorRole> = actor_roles.iter().find(|ar| ar.id == transaction.executor_id).cloned();
                let can_delete = !transactions_cloned.iter().any(|tr| tr.impediments.iter().any(|imp| imp.impeding_transaction_id == transaction.id));
                ui.add_enabled_ui(can_delete, |ui| {
                    if ui.button(RichText::new("❌").color(Color32::RED))
                        .on_disabled_hover_text("Used in an impediment")
                        .clicked() {
                        to_delete.push(t_index);
                    }
                });
                ui.add(egui::TextEdit::singleline(&mut transaction.t_id).min_size([50.0, 20.0].into()));
                ui.add(egui::TextEdit::singleline(&mut transaction.name).min_size([200.0, 20.0].into()));
                ui.add(egui::TextEdit::singleline(&mut transaction.product).min_size([200.0, 20.0].into()));
                // Initiator actor role combo
                egui::ComboBox::from_id_salt(format!("{}_{}_{}", "Initiator", transaction.id, t_index))
                    .selected_text(initiator.unwrap_or_default().name)
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                        ui.set_min_width(60.0);
                        for actor_role in actor_roles {
                            ui.selectable_value(&mut transaction.initiator_id, actor_role.id.clone(), actor_role.name.clone());
                        }
                    });
                // Executor actor role combo
                egui::ComboBox::from_id_salt(format!("{}_{}_{}", "Executor", transaction.id, t_index))
                    .selected_text(executor.unwrap_or_default().name)
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                        ui.set_min_width(60.0);
                        for actor_role in available_executors.clone() {
                            ui.selectable_value(&mut transaction.executor_id, actor_role.id.clone(), actor_role.name.clone());
                        }
                    });
                initiations_ui(ui, &transactions_cloned, &mut transaction);
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