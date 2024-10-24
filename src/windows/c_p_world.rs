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

            // Filters
            let mut t_insts_uuids: Vec<String> = execution.c_p_world.iter().map(|c_p_world_item| c_p_world_item.get_transaction_instance_id().to_string()).collect();
            t_insts_uuids.push(String::new());
            t_insts_uuids.sort();
            t_insts_uuids.dedup();
            let mut performers: Vec<String> = execution.c_p_world.iter().map(|c_p_world_item| c_p_world_item.get_performer().name.clone()).collect();
            performers.push(String::new());
            performers.sort();
            performers.dedup();
            ui.strong(" ");
            ui.strong(" ");
            egui::ComboBox::from_id_salt("C_P_World_Transaction_Instance_ID_filter")
                .selected_text(app_context.c_p_world_context.transaction_instance_id_filter.clone())
                .show_ui(ui, |ui| {
                    for uuid in t_insts_uuids {
                        ui.selectable_value(&mut app_context.c_p_world_context.transaction_instance_id_filter, uuid.to_string(), uuid.to_string());
                    }
                });
            egui::ComboBox::from_id_salt("C_P_World_Performer_filter")
                .selected_text(app_context.c_p_world_context.performer_filter.clone())
                .show_ui(ui, |ui| {
                    for performer in performers {
                        ui.selectable_value(&mut app_context.c_p_world_context.performer_filter, performer.clone(), performer.clone());
                    }
                });
            ui.strong(" ");
            ui.strong(" ");
            ui.end_row();


            app_context.hi_transaction_instance_id_o = None;
            for c_p_world_item in &execution.c_p_world {
                let t_inst_id_filter = &app_context.c_p_world_context.transaction_instance_id_filter;
                let performer_filter = &app_context.c_p_world_context.performer_filter;
                if (t_inst_id_filter.is_empty() || *t_inst_id_filter == c_p_world_item.get_transaction_instance_id().to_string()) &&
                   (performer_filter.is_empty() || *performer_filter == c_p_world_item.get_performer().name) {
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
            }
        });
}