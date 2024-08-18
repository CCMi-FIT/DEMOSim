#[cfg(target_arch = "wasm32")]
mod web_persistence {
    use web_sys::window;
    use serde_json;
    use crate::model::Model;

    pub fn save_model(model: &[Model]) {
        if let Ok(serialized) = serde_json::to_string(model) {
            if let Some(window) = window() {
                if let Some(local_storage) = window.local_storage().unwrap() {
                    let _ = local_storage.set_item("model", &serialized);
                }
            }
        }
    }

    pub fn load_model() -> Vec<Model> {
        if let Some(window) = window() {
            if let Some(local_storage) = window.local_storage().unwrap() {
                if let Ok(Some(data)) = local_storage.get_item("model") {
                    if let Ok(model) = serde_json::from_str(&data) {
                        return model;
                    }
                }
            }
        }
        Vec::new()
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod desktop_persistence {
    use std::fs::File;
    use std::io::{Read, Write};
    use serde_json;
    use crate::model::Model;

    pub fn save_model(model: &Model) -> std::io::Result<()> {
        let file_path = format!("{}.json", model.name);
        let json = serde_json::to_string_pretty(model)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_model<'a>() -> std::io::Result<Model> {
        let file_path = "model.json";
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let model: Model = serde_json::from_str(&contents)?;
        Ok(model)
    }
}

#[cfg(target_arch = "wasm32")]
pub use self::web_persistence::*;

#[cfg(not(target_arch = "wasm32"))]
pub use self::desktop_persistence::*;
