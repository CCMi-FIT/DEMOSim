use std::collections::HashMap;
use crate::execution::{Execution, TransactionInstanceId};
use crate::model::{CPAct, Model, SubjectId, TransactionId};
use crate::persistence::{load_model, save_model};
use crate::windows::EguiWindows;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct InitiateTransactionModalContext {
    pub parent_transaction_instance_id: Option<TransactionInstanceId>,
    pub initiated_transaction_id_o: Option<TransactionId>,
    pub requested_product: String,
    pub addressee_id_o: Option<SubjectId>,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SubjectContext {
    pub focused_subject_id_o: Option<SubjectId>,
    #[serde(skip)]
    pub selected_next_act: HashMap<TransactionInstanceId, CPAct>,
}

impl SubjectContext {
    #[inline]
    pub fn get_selected_next_act(&self, transaction_instance_id: &TransactionInstanceId) -> Option<&CPAct> {
        self.selected_next_act.get(transaction_instance_id)
    }

    #[inline]
    pub fn clear_selected_next_act(&mut self) {
        self.selected_next_act.clear();
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AppContext {
    pub model: Model,
    pub execution: Execution,
    pub initiate_transaction_modal_context: InitiateTransactionModalContext,
    pub subject_context: SubjectContext,
    pub hi_transaction_instance_id_o: Option<TransactionInstanceId>, // highlighted
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DemosimApp {
    egui_windows: EguiWindows,
    app_context: AppContext,
}

impl DemosimApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for DemosimApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("New model").clicked() {
                            self.app_context = Default::default();
                            ui.close_menu();
                        }
                        if ui.button("Load model...").clicked() {
                            if let Some(model) = load_model().unwrap() {
                                self.app_context.model = model;
                            }
                            ui.close_menu();
                        }
                        if ui.button("Save model...").clicked() {
                            save_model(&self.app_context.model).unwrap();
                            ui.close_menu();
                        }
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(170.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label("Model");
                ui.add_space(10.0);
                if ui.button("Actor Roles").clicked() {
                    self.egui_windows.actor_roles = true;
                }
                if ui.button("Transactions").clicked() {
                    self.egui_windows.transactions = true;
                }
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                ui.label("Implementation");
                ui.add_space(10.0);
                if ui.button("Subjects").clicked() {
                    self.egui_windows.subjects = true;
                }
                if ui.button("ADT").clicked() {
                    self.egui_windows.adt = true;
                }
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                ui.label("Execution");
                ui.add_space(10.0);
                if ui.button("Subjects Dashboard").clicked() {
                    self.egui_windows.subjects_dashboard = true;
                }
                if ui.button("Instances of Transactions").clicked() {
                    self.egui_windows.transactions_instances = true;
                }
                if ui.button("Coord/Prod World").clicked() {
                    self.egui_windows.c_p_world = true;
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                ui.add(egui::TextEdit::singleline(&mut self.app_context.model.name));
            });
            // ui.heading(self.model.name.clone());
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        self.egui_windows.windows(ctx, &mut self.app_context);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
