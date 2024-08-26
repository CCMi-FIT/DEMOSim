use crate::model::{Model, SubjectId};

mod actor_roles;
mod transactions;
mod subjects;
mod adt;
mod subjects_dashboard;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EguiWindows {
    pub actor_roles: bool,
    pub transactions: bool,
    pub subjects: bool,
    pub adt: bool,
    pub subjects_dashboard: bool,
}

impl Default for EguiWindows {
    fn default() -> Self {
        Self {
            transactions: false,
            actor_roles: false,
            subjects: false,
            adt: false,
            subjects_dashboard: false,
        }
    }
}

impl EguiWindows {
    pub fn windows(&mut self, ctx: &egui::Context, model: &mut Model, focused_subject_id: &mut Option<SubjectId>) {
        let Self {
            actor_roles,
            transactions,
            subjects,
            adt,
            subjects_dashboard,
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
        egui::Window::new("Subjects Dashboard")
            .open(subjects_dashboard)
            .vscroll(true)
            .show(ctx, |ui| {
                subjects_dashboard::subjects_dashboard_ui(ui, &model, focused_subject_id)
            });
    }
}

