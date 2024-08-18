use egui::{Id, Ui, Vec2};

use crate::model::{all_c_facts, CFact};

use super::button::CFactSelectorButtonState;

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
struct CFactSelectorPopupState {
    c_act: CFact,
    setup: bool,
}

pub(crate) struct CFactSelectorPopup<'a> {
    pub selection: &'a mut CFact,
    pub button_id: Id,
}

impl<'a> CFactSelectorPopup<'a> {
    /// Returns `true` if user pressed `Select` button.
    pub fn draw(&mut self, ui: &mut Ui) -> bool {
        let id = ui.make_persistent_id("c_act_selector");
        let mut popup_state = ui
            .data_mut(|data| data.get_persisted::<CFactSelectorPopupState>(id))
            .unwrap_or_default();
        if !popup_state.setup {
            popup_state.c_act = self.selection.clone();
            popup_state.setup = true;
            ui.data_mut(|data| data.insert_persisted(id, popup_state.clone()));
        }

        let (close, saved) = (false, false);
        let spacing = 2.0;
        ui.spacing_mut().item_spacing = Vec2::splat(spacing);

        for choice in all_c_facts().iter() {
            ui.radio_value(&mut popup_state.c_act, choice.clone(), choice.to_string());
        }

        if close {
            popup_state.setup = false;
            ui.data_mut(|data| {
                data.insert_persisted(id, popup_state);
                data.get_persisted_mut_or_default::<CFactSelectorButtonState>(self.button_id)
                    .selector_visible = false;
            });
        }

        saved && close
    }
}




