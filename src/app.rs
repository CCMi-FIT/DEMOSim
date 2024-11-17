use std::collections::HashMap;
use crate::execution::{Execution, TransactionInstanceId};
use crate::model::{CPAct, Model, SubjectId, TransactionId};
use crate::windows::EguiWindows;
use std::future::Future;
use std::sync::mpsc::{channel, Receiver, Sender};

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

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
    pub selected_next_act: HashMap<(SubjectId, TransactionInstanceId), CPAct>,
}

impl SubjectContext {
    #[inline]
    pub fn get_selected_next_act(&self, subject_id: &SubjectId, transaction_instance_id: &TransactionInstanceId) -> Option<&CPAct> {
        self.selected_next_act.get(&(subject_id.clone(), transaction_instance_id.clone()))
    }

    #[inline]
    pub fn clear_selected_next_act(&mut self, subject_id: &SubjectId, transaction_instance_id: &TransactionInstanceId) {
        self.selected_next_act.remove(&(subject_id.clone(), transaction_instance_id.clone()));
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CPWorldContext {
    pub transaction_instance_id_filter: String,
    pub performer_filter: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AppContext {
    pub model: Model,
    pub execution: Execution,
    pub initiate_transaction_modal_context: InitiateTransactionModalContext,
    pub subject_context: SubjectContext,
    pub c_p_world_context: CPWorldContext,
    pub hi_transaction_instance_id_o: Option<TransactionInstanceId>, // highlighted
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DemosimApp {
    #[serde(skip)]
    model_text_channel: (Sender<String>, Receiver<String>),
    egui_windows: EguiWindows,
    app_context: AppContext,
}

impl Default for DemosimApp {
    fn default() -> Self {
        Self {
            model_text_channel: channel(),
            egui_windows: EguiWindows::default(),
            app_context: AppContext::default(),
        }
    }
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

    fn try_load_model(&mut self) {
        if let Ok(model_text) = self.model_text_channel.1.try_recv() {
            let model: Model = ron::from_str(&model_text).unwrap();
            self.app_context.model = model;
        }
    }

}

impl eframe::App for DemosimApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.try_load_model();
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New model").clicked() {
                        self.app_context = Default::default();
                        ui.close_menu();
                    }
                    if ui.button("📂 Load model...").clicked() {
                        let sender = self.model_text_channel.0.clone();
                        let task = rfd::AsyncFileDialog::new()
                            .set_title("Load model")
                            .add_filter("DEMOsim", &["*.dms"])
                            .pick_file();
                        let ctx = ui.ctx().clone();
                        execute(async move {
                            let file = task.await;
                            if let Some(file) = file {
                                let text = file.read().await;
                                let _ = sender.send(String::from_utf8_lossy(&text).to_string());
                                ctx.request_repaint();
                            }
                        });
                        ui.close_menu();
                    }
                    if ui.button("💾 Save model...").clicked() {
                        let task = rfd::AsyncFileDialog::new()
                            .set_title("Save model")
                            .add_filter("DEMOsim", &["*.dms"])
                            .set_file_name(format!("{}.dms", self.app_context.model.name))
                            .save_file();
                        let model_text = ron::ser::to_string_pretty(&self.app_context.model, ron::ser::PrettyConfig::default()).unwrap();
                        execute(async move {
                            let file = task.await;
                            if let Some(file) = file {
                                _ = file.write(model_text.as_bytes()).await;
                            }
                        });
                        ui.close_menu();
                    }
                    // NOTE: no File->Quit on web pages!
                    let is_web = cfg!(target_arch = "wasm32");
                    if !is_web {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });
                ui.add_space(16.0);
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
        ui.label(format!("Version: {}. Powered by ", env!("CARGO_PKG_VERSION")));
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
