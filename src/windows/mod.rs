use crate::model::Model;

mod transactions;
mod actor_roles;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EguiWindows {
    pub transactions: bool,
    pub actor_roles: bool,
}

impl Default for EguiWindows {
    fn default() -> Self {
        Self {
            transactions: false,
            actor_roles: false,
        }
    }
}

impl EguiWindows {
    pub fn windows(&mut self, ctx: &egui::Context, model: &mut Model) {
        let Self {
            transactions,
            actor_roles,
        } = self;

        egui::Window::new("Transactions")
            .open(transactions)
            .vscroll(true)
            .show(ctx, |ui| {
                transactions::transactions_ui(ui, &model.actor_roles, &mut model.transactions)
            });
        egui::Window::new("Actor Roles")
            .open(actor_roles)
            .vscroll(true)
            .show(ctx, |ui| {
                actor_roles::actor_roles_ui(ui, &model.transactions, &mut model.actor_roles)
            });
    }
}

