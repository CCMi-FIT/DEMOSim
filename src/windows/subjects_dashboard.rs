use egui::TextWrapMode;
use crate::app::AppContext;
use crate::execution::{Execution, TransactionInstance, TransactionInstanceId};
use crate::model::{CPAct, CPFact, Impediment, Model, Subject, SubjectId, Transaction, TransactionId};

#[inline]
pub fn subjects_tabs_ui(ui: &mut egui::Ui, app_context: &mut AppContext) {
    let model = &app_context.model;
    let subject_context = &mut app_context.subject_context;
    if model.subjects.is_empty() {
        subject_context.focused_subject_id_o = None;
    } else {
        if subject_context.focused_subject_id_o.is_none() { subject_context.focused_subject_id_o = Some(model.subjects[0].id.clone())}
    }
    egui::TopBottomPanel::top("subjects_dashboard_top_panel")
        .resizable(false)
        .show_inside(ui, |ui| {
            if let Some(fp_id) = subject_context.focused_subject_id_o.clone() {
                let mut sel_fp_id = fp_id.clone();
                ui.horizontal_wrapped(|ui| {
                    for subject in &model.subjects {
                        ui.selectable_value(&mut sel_fp_id, subject.id.clone(), &subject.name);
                    }
                });
                subject_context.focused_subject_id_o = Some(sel_fp_id);
            }
        });
}

fn can_commit(model: &Model, execution: &Execution, transaction_instance: &TransactionInstance, act: &CPAct) -> bool {
    let impediments = &model.get_transaction(&transaction_instance.transaction_id).impediments;
    let impeding_transactions_instances: Vec<(&Impediment, &TransactionInstance)> =
        impediments.iter()
            .filter_map(|imp|
                execution.get_instances_of_transaction(&imp.impeding_transaction_id)
                    .into_iter().find(|t_i| t_i.parent_transaction_instance_id == Some(transaction_instance.id.clone()))
                    .map(|t_i| (imp, t_i))
            )
            .collect();
    let commit_possible = impediments.is_empty() || impediments.iter().all(|imp| {
        imp.impeded_act != *act ||
            impeding_transactions_instances.iter().find(
                |(imp1, t_i)| **imp1 == *imp && execution.get_c_p_world_item_by_fact(&t_i.id, &CPFact::CFact(imp.impeding_c_fact.clone())).is_some()
            ).is_some()
    });
    commit_possible
}


fn initiate_transactions_ui<F>(
    ui: &mut egui::Ui,
    startable_transactions: &Vec<&Transaction>,
    parent_transaction_instance_id: Option<TransactionInstanceId>,
    modal_opened: bool,
    open_modal: &mut F,
) where F: FnMut(Option<TransactionInstanceId>, TransactionId) {
    ui.horizontal(|ui| {
        for s_t in startable_transactions {
            ui.add_enabled_ui(!modal_opened, |ui| {
                if ui.button(format!("Request {}: {}", s_t.t_id, s_t.name))
                    .clicked() { open_modal(parent_transaction_instance_id.clone(), s_t.id.clone()); }
            });
        }
    });

}

#[inline]
fn startable_transactions_ui<F>(
    ui: &mut egui::Ui,
    app_context: &mut AppContext,
    subject_id: &SubjectId,
    parent_transaction_instance_id: Option<TransactionInstanceId>,
    modal_opened: bool,
    open_modal: &mut F,
) where F: FnMut(Option<TransactionInstanceId>, TransactionId) {
    let subject: &Subject = app_context.model.subjects.iter().find(|p| p.id == *subject_id).unwrap();
    let startable_transactions = app_context.model.startable_transactions(&subject);
    initiate_transactions_ui(ui, &startable_transactions, parent_transaction_instance_id, modal_opened, open_modal);
    ui.add_space(10.0);
    ui.separator();
}

#[inline]
fn agenda_ui<F>(
    ui: &mut egui::Ui,
    app_context: &mut AppContext,
    subject_id: &SubjectId,
    modal_opened: bool,
    mut open_modal: &mut F,
) where F: FnMut(Option<TransactionInstanceId>, TransactionId) {
    let model = &app_context.model;
    let execution = &mut app_context.execution;
    let subject_context = &mut app_context.subject_context;
    let agenda = execution.agenda_for(subject_id).clone();
    subject_context.clear_selected_next_act();
    egui::Grid::new("Subject's agenda")
        .striped(true)
        .spacing(&[10.0, 10.0])
        .show(ui, |ui| {
            ui.strong("Timestamp");
            ui.strong("Transaction");
            ui.strong("Performer");
            ui.strong("Fact");
            ui.strong("Product Instance");
            ui.strong("Act");
            ui.end_row();

            for agenda_item in &agenda {
                let transaction_instance = execution.get_transaction_instance(&agenda_item.transaction_instance_id).clone();
                let transaction = model.get_transaction(&transaction_instance.transaction_id);
                let performer = model.get_subject(&agenda_item.performer_id);
                let next_acts = agenda_item.fact.next_acts();
                let mut selected_next_act = subject_context.get_selected_next_act(&transaction_instance.id)
                    .unwrap_or(&next_acts[0].clone()).to_owned();
                let commit_enabled = can_commit(model, execution, &transaction_instance, &selected_next_act);

                ui.label(agenda_item.timestamp.to_string());
                ui.label(format!("{}: {}", transaction.t_id.to_string(), transaction.name.clone()));
                ui.label(performer.name.clone());
                ui.label(agenda_item.fact.to_string());
                ui.label(transaction_instance.product_instance.clone());
                egui::ComboBox::from_id_source(format!("Act for Fact {}", transaction_instance.id))
                    .selected_text(selected_next_act.to_string())
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                        ui.set_min_width(60.0);
                        for act in next_acts {
                            ui.selectable_value(&mut selected_next_act, act.clone(), act.to_string());
                        }
                    });
                ui.add_enabled_ui(commit_enabled, |ui| {
                    if ui.button("Commit")
                        .on_disabled_hover_text("Act is impeded")
                        .clicked() {
                        execution.process_new_fact(model, transaction_instance.id.clone(), subject_id.clone(), selected_next_act.to_fact());
                        execution.remove_agenda_item(agenda_item);
                    }
                });
                subject_context.selected_next_act.insert(transaction_instance.id.clone(), selected_next_act);
                let startable_subtransactions = Execution::startable_subtransactions(model, &transaction_instance, subject_id);
                initiate_transactions_ui(ui, &startable_subtransactions, Some(transaction_instance.id), modal_opened, &mut open_modal);
                ui.end_row();
            }
        });
}

#[inline]
pub fn subject_pane_ui<F>(
    ui: &mut egui::Ui,
    app_context: &mut AppContext,
    subject_id: &SubjectId,
    modal_opened: bool,
    mut open_modal: F,
) where F: FnMut(Option<TransactionInstanceId>, TransactionId) {
    ui.strong("Initiate transaction");
    ui.add_space(5.0);
    startable_transactions_ui(ui, app_context, subject_id, None, modal_opened, &mut open_modal);
    ui.strong("Agenda");
    ui.add_space(5.0);
    agenda_ui(ui, app_context, subject_id, modal_opened, &mut open_modal);
}

