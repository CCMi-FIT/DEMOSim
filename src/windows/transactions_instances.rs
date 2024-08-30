use egui::{Color32, RichText};
use crate::execution::TransactionInstance;
use crate::model::Model;

pub fn view(ui: &mut egui::Ui, model: &Model, transactions_instances: &mut Vec<TransactionInstance>) {
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
            ui.strong(" ");
            ui.end_row();
            for (index, t_i) in transactions_instances.iter_mut().enumerate() {
                let transaction = model.get_transaction(&t_i.transaction_id);
                let initiator_subject = model.get_subject(&t_i.initiator);
                let executor_subject = model.get_subject(&t_i.executor);
                ui.label(t_i.id.clone().to_string());
                ui.label(transaction.t_id.clone());
                ui.label(t_i.product_instance.clone());
                ui.label(initiator_subject.name.clone());
                ui.label(executor_subject.name.clone());
                if ui.button(RichText::new("❌").color(Color32::RED)).clicked() {
                    to_delete.push(index);
                }
                ui.end_row();
            }
        });
    for index in to_delete.into_iter().rev() {
        transactions_instances.remove(index);
    }
}

