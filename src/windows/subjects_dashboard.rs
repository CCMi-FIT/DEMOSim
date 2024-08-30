use crate::app::AppContext;
use crate::model::{Subject, SubjectId, Model, TransactionId};

pub fn subjects_tabs_ui(ui: &mut egui::Ui, model: &Model, app_context: &mut AppContext) {
    if model.subjects.is_empty() {
        app_context.focused_subject_id_o = None;
    } else {
        if app_context.focused_subject_id_o.is_none() { app_context.focused_subject_id_o = Some(model.subjects[0].id.clone())}
    }
    egui::TopBottomPanel::top("subjects_dashboard_top_panel")
        .resizable(false)
        .show_inside(ui, |ui| {
            if let Some(fp_id) = app_context.focused_subject_id_o.clone() {
                let mut sel_fp_id = fp_id.clone();
                ui.horizontal_wrapped(|ui| {
                    for subject in &model.subjects {
                        ui.selectable_value(&mut sel_fp_id, subject.id.clone(), &subject.name);
                    }
                });
                app_context.focused_subject_id_o = Some(sel_fp_id);
            }
        });
}

pub fn subject_pane_ui<F>(ui: &mut egui::Ui, model: &Model, subject_id: &SubjectId, modal_opened: bool, mut setup_modal: F)
where F: FnMut(TransactionId) {
    let subject: &Subject = model.subjects.iter().find(|p| p.id == *subject_id).unwrap();
    let startable_transactions = model.startable_transactions(&subject);
    ui.strong("Initiate transaction");
    ui.add_space(5.0);
    ui.horizontal(|ui| {
        for s_t in startable_transactions {
            ui.add_enabled_ui(!modal_opened, |ui| {
                if ui.button(s_t.name.clone())
                    .clicked() { setup_modal(s_t.id.clone())}
            });
        }
        ui.add_space(5.0);
    });
    ui.add_space(10.0);
    ui.separator();
}

