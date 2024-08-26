use crate::model::{Subject, SubjectId, Model};

fn render_subject(ui: &mut egui::Ui, model: &Model, subject_id: &SubjectId) {
    let subject: &Subject = model.subjects.iter().find(|p| p.id == *subject_id).unwrap();
    let startable_transactions = model.startable_transactions(&subject);
    ui.strong("Initiate transaction");
    ui.add_space(5.0);
    ui.horizontal(|ui| {
        for s_t in startable_transactions {
            if ui.button(s_t.name.clone()).clicked() {
               //
            }
        }
        ui.add_space(5.0);
    });
    ui.add_space(10.0);
    ui.separator();
}

pub fn subjects_dashboard_ui(ui: &mut egui::Ui, model: &Model, focused_subject_id: &mut Option<SubjectId>) {
    if model.subjects.is_empty() {
        *focused_subject_id = None;
    } else {
        if focused_subject_id.is_none() { *focused_subject_id = Some(model.subjects[0].id.clone())}
    }
    egui::TopBottomPanel::top("subjects_dashboard_top_panel")
        .resizable(false)
        .show_inside(ui, |ui| {
            if let Some(fp_id) = focused_subject_id {
                let mut sel_fp_id = fp_id.clone();
                ui.horizontal_wrapped(|ui| {
                    for subject in &model.subjects {
                        ui.selectable_value(&mut sel_fp_id, subject.id.clone(), &subject.name);
                    }
                });
                *focused_subject_id = Some(sel_fp_id);
            }
        });
    ui.add_space(10.0);
    if let Some(fp_id) = focused_subject_id {
        render_subject(ui, model, fp_id);
    }
}
