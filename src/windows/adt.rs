use eframe::epaint::Color32;
use egui::RichText;
use crate::model::{Subject, Adt, AdtOption, ActorRole};

pub fn adt_ui(ui: &mut egui::Ui, actor_roles: &Vec<ActorRole>, subjects: &Vec<Subject>, adt: &mut Adt) {
    egui::Grid::new("ADT")
        .striped(true)
        .spacing(&[5.0, 5.0])
        .show(ui, |ui| {
            ui.strong(" ");
            for actor_role in actor_roles {
                let color: Color32 = if adt.is_mapped(&actor_role.id) { Color32::GREEN } else { Color32::RED };
                ui.strong(RichText::new(actor_role.name.clone()).color(color));
            }
            ui.end_row();

            for subject in subjects.iter() {
                ui.strong(subject.name.clone());
                for actor_role in actor_roles {
                    let mut choice: String = adt.mappings
                        .get(&(actor_role.id.clone(), subject.id.clone()))
                        .map_or_else(|| "".to_string(), |adt_option| adt_option.to_string());
                    ui.add(egui::TextEdit::singleline(&mut choice));
                    if let Some(adt_option) = AdtOption::from_str(&choice) {
                      adt.mappings.insert((actor_role.id.clone(), subject.id.clone()), adt_option);
                    } else {
                      adt.mappings.remove(&(actor_role.id.clone(), subject.id.clone()));
                    }
                }
                ui.end_row();
            }
        });
    ui.add_space(20.0);
    ui.label(RichText::new("A = Authorized, D = Delegated").color(Color32::DARK_GRAY));
}
