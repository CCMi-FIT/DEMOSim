use egui::TextWrapMode;
use crate::app::AppContext;
use crate::execution::{Execution, TransactionInstanceId};
use crate::model::{CPAct, Model, SubjectId, Transaction, TransactionId};
use crate::model::CAct::Request;

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


fn initiate_transactions_ui<F>(
    ui: &mut egui::Ui,
    model: &Model,
    execution: &Execution,
    startable_transactions: &Vec<&Transaction>,
    parent_transaction_instance_id_o: Option<TransactionInstanceId>,
    modal_opened: bool,
    open_modal: &mut F,
) where F: FnMut(Option<TransactionInstanceId>, TransactionId) {
    ui.horizontal(|ui| {
        for s_t in startable_transactions {
            let impediments_msgs_o = parent_transaction_instance_id_o.as_ref()
                .and_then(|parent_tran_inst_id| execution.get_act_impediments(model, s_t, parent_tran_inst_id.clone(), &CPAct::CAct(Request)))
                .map(|msgs| msgs.join("\n"));
            let enabled = !modal_opened && impediments_msgs_o.is_none();
            ui.add_enabled_ui(enabled, |ui| {
                if ui.button(format!("Request {}: {}", s_t.t_id, s_t.name))
                    .on_disabled_hover_text(impediments_msgs_o.clone().unwrap_or_default())
                    .clicked() { open_modal(parent_transaction_instance_id_o.clone(), s_t.id.clone()); }
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
    let startable_transactions = app_context.model.directly_startable_transactions(subject_id);
    initiate_transactions_ui(ui, &app_context.model, &app_context.execution, &startable_transactions, parent_transaction_instance_id, modal_opened, open_modal);
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
                let transaction_instance_parent = transaction_instance.parent_transaction_instance_id.clone().unwrap_or_else(|| transaction_instance.id.clone());
                let transaction = model.get_transaction(&transaction_instance.transaction_id);
                let performer = model.get_subject(&agenda_item.performer_id);
                let next_acts = agenda_item.fact.next_acts();
                let mut selected_next_act = subject_context.get_selected_next_act(&subject_id, &transaction_instance.id)
                    .unwrap_or(&next_acts[0].clone()).to_owned();
                let mut committed = false;
                let impediments_msgs_o = execution.get_act_impediments(model, &transaction, transaction_instance_parent, &selected_next_act).map(|msgs| msgs.join("\n"));

                ui.label(agenda_item.timestamp.to_string());
                ui.label(format!("{}: {}", transaction.t_id.to_string(), transaction.name.clone()));
                ui.label(performer.name.clone());
                ui.label(agenda_item.fact.to_string());
                ui.label(transaction_instance.product_instance.clone());
                egui::ComboBox::from_id_salt(format!("Act for Fact {}", transaction_instance.id))
                    .selected_text(selected_next_act.to_string())
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                        ui.set_min_width(60.0);
                        for act in next_acts {
                            ui.selectable_value(&mut selected_next_act, act.clone(), act.to_string());
                        }
                    });
                ui.add_enabled_ui(impediments_msgs_o.is_none(), |ui| {
                    if ui.button("Commit")
                        .on_disabled_hover_text(impediments_msgs_o.unwrap_or_default())
                        .clicked() {
                            execution.process_new_fact(model, transaction_instance.id.clone(), subject_id.clone(), selected_next_act.to_fact());
                            execution.remove_agenda_item(agenda_item);
                            committed = true;
                        }
                });
                if committed {
                    subject_context.clear_selected_next_act(subject_id, &transaction_instance.id);
                } else {
                    subject_context.selected_next_act.insert((subject_id.clone(), transaction_instance.id.clone()), selected_next_act);
                }
                let startable_subtransactions = Execution::startable_subtransactions(model, execution, &transaction_instance, subject_id);
                initiate_transactions_ui(ui, model, execution, &startable_subtransactions, Some(transaction_instance.id), modal_opened, &mut open_modal);
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

