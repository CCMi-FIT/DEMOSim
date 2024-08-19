use crate::model::{Model, PerformerId};

mod actor_roles;
mod transactions;
mod performers;
mod adt;
mod performers_dashboard;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EguiWindows {
    pub actor_roles: bool,
    pub transactions: bool,
    pub performers: bool,
    pub adt: bool,
    pub performers_dashboard: bool,
}

impl Default for EguiWindows {
    fn default() -> Self {
        Self {
            transactions: false,
            actor_roles: false,
            performers: false,
            adt: false,
            performers_dashboard: false,
        }
    }
}

impl EguiWindows {
    pub fn windows(&mut self, ctx: &egui::Context, model: &mut Model, focused_performer_id: &mut Option<PerformerId>) {
        let Self {
            actor_roles,
            transactions,
            performers,
            adt,
            performers_dashboard,
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
        egui::Window::new("Performers")
            .open(performers)
            .vscroll(true)
            .show(ctx, |ui| {
                performers::performers_ui(ui, &mut model.performers)
            });
        egui::Window::new("ADT")
            .open(adt)
            .vscroll(true)
            .show(ctx, |ui| {
                adt::adt_ui(ui, &model.transactions, &model.performers, &mut model.adt)
            });
        egui::Window::new("Performers Dashboard")
            .open(performers_dashboard)
            .vscroll(true)
            .show(ctx, |ui| {
                performers_dashboard::performers_dashboard_ui(ui, focused_performer_id, &model.transactions, &model.performers, &model.adt)
            });
    }
}

