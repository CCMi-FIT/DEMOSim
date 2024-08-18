use egui::{Area, Button, Frame, InnerResponse, Key, Order, Ui, Widget};
use crate::components::c_fact_selector::popup::CFactSelectorPopup;

use crate::model::CFact;

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct CFactSelectorButtonState {
    pub selector_visible: bool,
}

pub struct CFactSelectorButton<'a> {
    selection: &'a mut CFact,
    id_source: Option<&'a str>,
}

impl<'a> CFactSelectorButton<'a> {
    pub fn new(selection: &'a mut CFact) -> Self {
        Self {
            selection,
            id_source: None,
        }
    }

    /// Add id source.
    /// Must be set if multiple date selector buttons are in the same Ui.
    #[inline]
    pub fn id_source(mut self, id_source: &'a str) -> Self {
        self.id_source = Some(id_source);
        self
    }
}

impl<'a> Widget for CFactSelectorButton<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let id = ui.make_persistent_id(self.id_source);
        let mut button_state = ui
            .data_mut(|data| data.get_persisted::<CFactSelectorButtonState>(id))
            .unwrap_or_default();

        let button = Button::new("Select C-Act");
        let mut button_response = ui.add(button);
        if button_response.clicked() {
            button_state.selector_visible = true;
            ui.data_mut(|data| data.insert_persisted(id, button_state.clone()));
        }

        if button_state.selector_visible {
            let width = 333.0;
            let mut pos = button_response.rect.left_bottom();
            let width_with_padding = width
                + ui.style().spacing.item_spacing.x
                + ui.style().spacing.window_margin.left
                + ui.style().spacing.window_margin.right;
            if pos.x + width_with_padding > ui.clip_rect().right() {
                pos.x = button_response.rect.right() - width_with_padding;
            }

            // Check to make sure the selector never is displayed out of window
            pos.x = pos.x.max(ui.style().spacing.window_margin.left);

            let InnerResponse {
                inner: saved,
                response: area_response,
            } = Area::new(ui.make_persistent_id(self.id_source))
                .order(Order::Foreground)
                .fixed_pos(pos)
                .constrain_to(ui.ctx().screen_rect())
                .show(ui.ctx(), |ui| {
                    let frame = Frame::popup(ui.style());
                    frame
                        .show(ui, |ui| {
                            ui.set_min_width(width);
                            ui.set_max_width(width);

                            CFactSelectorPopup {
                                selection: self.selection,
                                button_id: id,
                            }
                                .draw(ui)
                        })
                        .inner
                });

            if saved {
                button_response.mark_changed();
            }

            if !button_response.clicked()
                && (ui.input(|i| i.key_pressed(Key::Escape)) || area_response.clicked_elsewhere())
            {
                button_state.selector_visible = false;
                ui.data_mut(|data| data.insert_persisted(id, button_state));
            }
        }

        button_response
    }
}
