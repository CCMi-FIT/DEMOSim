use crate::app::{AppContext, InitiateTransactionModalContext};

mod actor_roles;
mod transactions;
mod subjects;
mod adt;
mod subjects_dashboard;
mod transaction_initiate_modal;
mod transactions_instances;
mod c_p_world;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct EguiWindows {
    pub actor_roles: bool,
    pub transactions: bool,
    pub subjects: bool,
    pub adt: bool,
    pub subjects_dashboard: bool,
    pub transaction_initiate_modal: bool,
    pub transactions_instances: bool,
    pub c_p_world: bool,
}

impl EguiWindows {
    pub fn windows(&mut self, ctx: &egui::Context, app_context: &mut AppContext) {
        let Self {
            actor_roles,
            transactions,
            subjects,
            adt,
            subjects_dashboard,
            transaction_initiate_modal,
            transactions_instances,
            c_p_world,
        } = self;
        let model = &mut app_context.model;

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
                transactions::transactions_ui(ui, &mut app_context.transaction_context, &model.actor_roles, &mut model.transactions)
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
        let mut new_initiate_transaction_modal_context_o = None;
        egui::Window::new("Subjects Dashboard")
            .open(subjects_dashboard)
            .vscroll(true)
            .show(ctx, |ui| {
                subjects_dashboard::subjects_tabs_ui(ui, app_context);
                ui.add_space(10.0);
                if let Some(fp_id) = app_context.subject_context.focused_subject_id_o.clone() {
                    subjects_dashboard::subject_pane_ui(
                        ui,
                        app_context,
                        &fp_id,
                        transaction_initiate_modal.clone(),
                        |parent_transaction_instance_id, transaction_id| {
                            new_initiate_transaction_modal_context_o = Some(InitiateTransactionModalContext {
                                parent_transaction_instance_id,
                                initiated_transaction_id_o: Some(transaction_id),
                                requested_product: String::new(),
                                addressee_id_o: None,
                            });
                        },
                    );
                }
            });
        if let Some(new_initiate_transaction_modal_context) = new_initiate_transaction_modal_context_o {
            app_context.initiate_transaction_modal_context = new_initiate_transaction_modal_context;
            transaction_initiate_modal_open_request = true;
        }
        egui::Window::new("Transaction Initiation")
            .open(transaction_initiate_modal)
            .vscroll(true)
            .show(ctx, |ui| {
                transaction_initiate_modal::view(
                    ui,
                    app_context,
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
                transactions_instances::view(ui, app_context)
            });
        egui::Window::new("Coordination / Production World")
            .open(c_p_world)
            .vscroll(true)
            .show(ctx, |ui| {
                c_p_world::view(ui, app_context)
            });
    }
}

