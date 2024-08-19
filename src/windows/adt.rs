use eframe::epaint::Color32;
use egui::RichText;
use crate::model::{Transaction, Performer, Adt, AdtOption};

pub fn adt_ui(ui: &mut egui::Ui, transactions: &Vec<Transaction>, performers: &Vec<Performer>, adt: &mut Adt) {
    egui::Grid::new("ADT")
        .striped(true)
        .spacing(&[5.0, 5.0])
        .show(ui, |ui| {
            ui.strong("TK/Performer");
            for transaction in transactions {
                let color: Color32 = if adt.is_mapped(&transaction.id) { Color32::GREEN } else { Color32::RED };
                ui.strong(RichText::new(transaction.t_id.clone()).color(color));
            }
            ui.end_row();

            for performer in performers.iter() {
                ui.strong(performer.name.clone());
                for transaction in transactions {
                    let mut choice: String = adt.mappings
                        .get(&(transaction.id.clone(), performer.id.clone()))
                        .map_or_else(|| "".to_string(), |adt_option| adt_option.to_string());
                    ui.add(egui::TextEdit::singleline(&mut choice));
                    if let Some(adt_option) = AdtOption::from_str(&choice) {
                      adt.mappings.insert((transaction.id.clone(), performer.id.clone()), adt_option);
                    }
                }
                ui.end_row();
            }
        });
    ui.add_space(20.0);
    RichText::new("T1").color(Color32::RED);
}
