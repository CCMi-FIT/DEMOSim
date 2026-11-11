use crate::app::AppContext;
use crate::execution::TransactionInstance;
use egui::TextWrapMode;

pub fn view<F: FnMut()>(ui: &mut egui::Ui, app_context: &mut AppContext, mut close_modal: F) {
    let model = &app_context.model;
    let execution = &mut app_context.execution;
    let modal_context = &mut app_context.initiate_transaction_modal_context;
    // Bind the dialog to its opening actor, including dialogs restored from older state.
    if modal_context.initiator_id_o.is_none() {
        modal_context.initiator_id_o = app_context.subject_context.focused_subject_id_o.clone();
    }
    if let Some(focused_subject_id) = modal_context.initiator_id_o.clone() {
        if let Some(initiated_transaction_id) = modal_context.initiated_transaction_id_o.clone() {
            let Some(performer) = model.subjects.iter().find(|s| s.id == focused_subject_id) else {
                ui.label(
                    "The initiating subject no longer exists. Close this dialog and start again.",
                );
                return;
            };
            let Some(transaction) = model
                .transactions
                .iter()
                .find(|t| t.id == initiated_transaction_id)
            else {
                ui.label(
                    "The transaction kind no longer exists. Close this dialog and start again.",
                );
                return;
            };
            let adt_options = model.adt.get_adt_options_for_role(&transaction.executor_id);
            egui::Grid::new("Subjects")
                .striped(true)
                .spacing([5.0, 5.0])
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
            ui.add(
                egui::TextEdit::singleline(&mut modal_context.requested_product)
                    .min_size([200.0, 20.0].into()),
            );
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Addressee:");
                let selected_addressee_o = modal_context
                    .addressee_id_o
                    .as_ref()
                    .and_then(|s_id| model.subjects.iter().find(|s| &s.id == s_id));
                egui::ComboBox::from_id_salt(format!("Addressee selection for {}", performer.id))
                    .selected_text(match selected_addressee_o {
                        Some(selected_addressee) => selected_addressee.name.clone(),
                        None => String::new(),
                    })
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                        ui.set_min_width(60.0);
                        for (subject_id, adt_option) in adt_options {
                            let Some(possible_addressee) =
                                model.subjects.iter().find(|s| &s.id == subject_id)
                            else {
                                continue;
                            };
                            ui.selectable_value(
                                &mut modal_context.addressee_id_o,
                                Some(subject_id.clone()),
                                format!("{} ({})", possible_addressee.name, adt_option),
                            );
                        }
                    });
            });
            ui.add_space(20.0);
            let request = modal_context.addressee_id_o.as_ref().map(|addressee| {
                TransactionInstance::new(
                    modal_context.parent_transaction_instance_id.clone(),
                    transaction.id.clone(),
                    modal_context.requested_product.clone(),
                    performer.id.clone(),
                    addressee.clone(),
                )
            });
            let validation = request
                .as_ref()
                .map(|request| execution.validate_request(model, request));
            let enabled = matches!(validation, Some(Ok(())));
            if let Some(Err(error)) = validation {
                ui.label(error);
            }
            ui.add_enabled_ui(enabled, |ui| {
                if ui.button("Request product").clicked() {
                    if let Some(request) = request {
                        match execution.add_transaction_instance(model, request) {
                            Ok(()) => close_modal(),
                            Err(error) => app_context.error = Some(error),
                        }
                    }
                }
            });
        }
    }
}
