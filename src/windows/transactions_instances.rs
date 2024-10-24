use egui::{Color32, RichText};
use crate::app::AppContext;
use crate::model::{CFact, CPFact};

pub fn view(ui: &mut egui::Ui, app_context: &mut AppContext) {
    let model = &app_context.model;
    let execution = &mut app_context.execution;
    let mut to_delete = Vec::new();
    egui::Grid::new("Transactions Instances")
        .striped(true)
        .spacing(&[10.0, 10.0])
        .show(ui, |ui| {
            ui.strong("Transaction Instance ID");
            ui.strong("Transaction");
            ui.strong("Parent TI ID");
            ui.strong("Product Instance");
            ui.strong("Initiator Subject");
            ui.strong("Executor Subject");
            ui.strong("Last Fact");
            ui.end_row();
            for t_i in execution.transactions_instances.iter() {
                let transaction = model.get_transaction(&t_i.transaction_id);
                let initiator_subject = model.get_subject(&t_i.initiator_id);
                let executor_subject = model.get_subject(&t_i.executor_id);
                let last_fact = execution.get_facts_for_transaction_instance(&t_i.id).last().unwrap().to_fact();
                let render_label = |ui: &mut egui::Ui, text: String| -> egui::Response {
                    match last_fact {
                        CPFact::CFact(CFact::Accepted) => ui.colored_label(Color32::GREEN, text),
                        _ => ui.label(text),
                    }
                };

                let id_label = render_label(ui, t_i.id.clone().to_string());
                if app_context.hi_transaction_instance_id_o == Some(t_i.id.clone()) {
                    id_label.highlight();
                }
                render_label(ui, format!("{}: {}", transaction.t_id.clone(), transaction.name.clone()));
                render_label(ui, match &t_i.parent_transaction_instance_id {
                    None => "--".to_string(),
                    Some(id) => id.to_string(),
                });
                render_label(ui, t_i.product_instance.clone());
                render_label(ui, initiator_subject.name.clone());
                render_label(ui, executor_subject.name.clone());
                render_label(ui, last_fact.to_string());
                if ui.button(RichText::new("❌").color(Color32::RED)).clicked() {
                    to_delete.push(t_i.id.clone());
                }
                ui.end_row();
            }
        });
    for transaction_instance_id in to_delete.into_iter().rev() {
        execution.delete_transaction_instance(&transaction_instance_id);
    }
}

