use crate::model::{Transaction, Adt, Performer, PerformerId};

fn render_performer(ui: &mut egui::Ui, performers: &Vec<Performer>, performer_id: &PerformerId) {
    let performer: &Performer = performers.iter().find(|p| p.id == *performer_id).unwrap();
    ui.label(performer.name.clone());
}

pub fn performers_dashboard_ui(ui: &mut egui::Ui, focused_performer_id: &mut Option<PerformerId>, transactions: &Vec<Transaction>, performers: &Vec<Performer>, adt: &Adt) {
    if performers.is_empty() {
        *focused_performer_id = None;
    } else {
        if focused_performer_id.is_none() { *focused_performer_id = Some(performers[0].id.clone())}
    }
    egui::TopBottomPanel::top("performers_dashboard_top_panel")
        .resizable(false)
        .show_inside(ui, |ui| {
            if let Some(fp_id) = focused_performer_id {
                let mut sel_fp_id = fp_id.clone();
                ui.horizontal_wrapped(|ui| {
                    for performer in performers {
                        ui.selectable_value(&mut sel_fp_id, performer.id.clone(), &performer.name);
                    }
                });
                *focused_performer_id = Some(sel_fp_id);
            }
        });
    ui.add_space(20.0);
    if let Some(fp_id) = focused_performer_id {
        render_performer(ui, performers, fp_id);
    }
}
