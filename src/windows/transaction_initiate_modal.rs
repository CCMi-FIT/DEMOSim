use egui::TextWrapMode;
use crate::app::AppContext;
use crate::model::Model;
use crate::execution::{Execution, TransactionInstance};

pub fn view<F: FnMut()>(
    ui: &mut egui::Ui,
    model: &Model,
    app_context: &mut AppContext,
    execution: &mut Execution,
    mut close_modal: F,
) {
    if let Some(focused_subject_id) = app_context.focused_subject_id_o.clone() {
        if let Some(initiated_transaction_id) = app_context.initiated_transaction_id_o.clone() {
            let performer = model.get_subject(&focused_subject_id);
            let transaction = model.get_transaction(&initiated_transaction_id);
            let executor = model.get_actor_role(&transaction.executor_id);
            let adt_options = model.adt.get_adt_options_for_role(&executor.id);
            egui::Grid::new("Subjects")
                .striped(true)
                .spacing(&[5.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Performer:");
                    ui.strong(performer.name.clone());
                    ui.end_row();
                    ui.label("Transaction:");
                    ui.strong(transaction.t_id.clone());
                    ui.end_row();
                    ui.label("Product:");
                    ui.strong(transaction.product.clone());
                    ui.end_row();
                });
            ui.add_space(10.0);
            ui.label("Requested product:");
            ui.add(egui::TextEdit::singleline(&mut app_context.requested_product).min_size([200.0, 20.0 ].into()));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Addressee:");
                let selected_addressee_o = app_context.addressee_id_o.clone().map(|s_id| model.get_subject(&s_id));
                egui::ComboBox::from_id_source(format!("Addressee selection for {}", performer.id))
                    .selected_text(match selected_addressee_o { Some(selected_addressee) => selected_addressee.name.clone(), None => String::new() })
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                        ui.set_min_width(60.0);
                        for (subject_id, adt_option) in adt_options {
                            let possible_addressee = model.get_subject(&subject_id);
                            ui.selectable_value(&mut app_context.addressee_id_o, Some(subject_id.clone()), format!("{} ({})", possible_addressee.name, adt_option));
                        }
                    });

            });
            ui.add_space(20.0);
            ui.add_enabled_ui(!app_context.requested_product.is_empty() && app_context.addressee_id_o.is_some(), |ui| {
                if ui.button("Request product").clicked() {
                    let t_i = TransactionInstance::new(transaction.id.clone(), app_context.requested_product.clone(), performer.id.clone(), app_context.addressee_id_o.clone().unwrap());
                    execution.transaction_instances.push(t_i);
                    close_modal();
                }
            });
        }
    }
}
