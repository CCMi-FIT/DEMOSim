use egui::{Color32, RichText};
use crate::app::AppContext;

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
            ui.strong("Product Instance");
            ui.strong("Initiator Subject");
            ui.strong("Executor Subject");
            // ui.strong("Last Fact");
            ui.end_row();
            for t_i in execution.transactions_instances.iter_mut() {
                let transaction = model.get_transaction(&t_i.transaction_id);
                let initiator_subject = model.get_subject(&t_i.initiator_id);
                let executor_subject = model.get_subject(&t_i.executor_id);

                let id_label = ui.label(t_i.id.clone().to_string());
                if app_context.hi_transaction_instance_id_o == Some(t_i.id.clone()) {
                    id_label.highlight();
                }
                ui.label(format!("{}: {}", transaction.t_id.clone(), transaction.name.clone()));
                ui.label(t_i.product_instance.clone());
                ui.label(initiator_subject.name.clone());
                ui.label(executor_subject.name.clone());
                //TODO:
                // ui.label(execution.get_facts_for_transaction_instance(&t_i.id).clone().last().to_fact().to_string());
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

