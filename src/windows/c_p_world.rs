use crate::app::AppContext;
use crate::execution::CPWorldItem;

pub fn view(ui: &mut egui::Ui, app_context: &mut AppContext) {
    let execution = &app_context.execution;
    egui::Grid::new("C/P World")
        .striped(true)
        .spacing(&[10.0, 10.0])
        .show(ui, |ui| {
            ui.strong("C/P");
            ui.strong("Timestamp");
            ui.strong("Transaction Instance ID");
            ui.strong("Performer");
            ui.strong("Addressee");
            ui.strong("Fact");
            ui.end_row();

            app_context.hi_transaction_instance_id_o = None;
            for c_p_world_item in &execution.c_p_world {
                use CPWorldItem::*;
                match c_p_world_item {
                    PWorldItem(p_world_item) => {
                        ui.label("P");
                        ui.label(p_world_item.timestamp.to_string());
                        if ui.label(p_world_item.transaction_instance_id.to_string()).hovered() {
                            app_context.hi_transaction_instance_id_o = Some(p_world_item.transaction_instance_id.clone());
                        }
                        ui.label(p_world_item.performer.name.to_string());
                        ui.label(" ");
                        ui.label(" ");
                    },
                    CWorldItem(c_world_item) => {
                        ui.label("C");
                        ui.label(c_world_item.timestamp.to_string());
                        if ui.label(c_world_item.transaction_instance_id.to_string()).hovered() {
                            app_context.hi_transaction_instance_id_o = Some(c_world_item.transaction_instance_id.clone());
                        }
                        ui.label(c_world_item.performer.name.to_string());
                        ui.label(c_world_item.addressee.name.to_string());
                        ui.label(c_world_item.fact.to_string());
                    }
                }
                ui.end_row();
            }
        });
}