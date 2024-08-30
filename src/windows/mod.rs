use crate::app::AppContext;
use crate::execution::Execution;
use crate::model::Model;

mod actor_roles;
mod transactions;
mod subjects;
mod adt;
mod subjects_dashboard;
mod transaction_initiate_modal;
mod transactions_instances;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct EguiWindows {
    pub actor_roles: bool,
    pub transactions: bool,
    pub subjects: bool,
    pub adt: bool,
    pub subjects_dashboard: bool,
    pub transaction_initiate_modal: bool,
    pub transactions_instances: bool,
}

impl EguiWindows {
    pub fn windows(&mut self, ctx: &egui::Context, app_context: &mut AppContext, model: &mut Model, execution: &mut Execution) {
        let Self {
            actor_roles,
            transactions,
            subjects,
            adt,
            subjects_dashboard,
            transaction_initiate_modal,
            transactions_instances,
        } = self;

        egui::Window::new("Actor Roles")
            .open(actor_roles)
            .vscroll(true)
            .show(ctx, |ui| {
                actor_roles::actor_roles_ui(ui, &model.transactions, &mut model.actor_roles)
            });
        egui::Window::new("Transactions")
            .open(transactions)
            .vscroll(true)
            .show(ctx, |ui| {
                transactions::transactions_ui(ui, &model.actor_roles, &mut model.transactions)
            });
        egui::Window::new("Subjects")
            .open(subjects)
            .vscroll(true)
            .show(ctx, |ui| {
                subjects::subjects_ui(ui, &mut model.subjects)
            });
        egui::Window::new("ADT")
            .open(adt)
            .vscroll(true)
            .show(ctx, |ui| {
                adt::adt_ui(ui, &model.actor_roles, &model.subjects, &mut model.adt)
            });
        let mut transaction_initiate_modal_open_request = false;
        let mut transaction_initiate_modal_close_request = false;
        egui::Window::new("Subjects Dashboard")
            .open(subjects_dashboard)
            .vscroll(true)
            .show(ctx, |ui| {
                subjects_dashboard::subjects_tabs_ui(ui, &model, app_context);
                ui.add_space(10.0);
                if let Some(fp_id) = app_context.focused_subject_id_o.clone() {
                    subjects_dashboard::subject_pane_ui(
                        ui,
                        model,
                        &fp_id,
                        transaction_initiate_modal.clone(),
                        |transaction_id| {
                            app_context.initiated_transaction_id_o = Some(transaction_id);
                            app_context.requested_product = String::new();
                            app_context.addressee_id_o = None;
                            transaction_initiate_modal_open_request = true;
                        },
                    );
                }
            });
        egui::Window::new("Transaction Initiation")
            .open(transaction_initiate_modal)
            .vscroll(true)
            .show(ctx, |ui| {
                transaction_initiate_modal::view(
                    ui,
                    &model,
                    app_context,
                    execution,
                    || { transaction_initiate_modal_close_request = true; }
                )
            });
        if transaction_initiate_modal_open_request {
            self.transaction_initiate_modal = true;
        }
        if transaction_initiate_modal_close_request {
            self.transaction_initiate_modal = false;
        }
        egui::Window::new("Instances of Transactions")
            .open(transactions_instances)
            .vscroll(true)
            .show(ctx, |ui| {
                transactions_instances::view(ui, model, &mut execution.transaction_instances)
            });
    }
}

