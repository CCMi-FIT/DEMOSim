#[cfg(target_arch = "wasm32")]
mod web_persistence {
    use web_sys::window;
    use crate::model::Model;

    pub fn save_model(model: &Model) -> std::io::Result<()> {
        // if let Ok(serialized) = ron::ser::to_string_pretty(model, ron::ser::PrettyConfig::default()) {
        //     if let Some(window) = window() {
        //         if let Some(local_storage) = window.local_storage().unwrap() {
        //             let _ = local_storage.set_item("model", &serialized);
        //         }
        //     }
        // }
        Ok(())
    }

    pub fn load_model() -> std::io::Result<Option<Model>> {
    //     if let Some(window) = window() {
    //         if let Some(local_storage) = window.local_storage().unwrap() {
    //             if let Ok(Some(data)) = local_storage.get_item("model") {
    //                 return ron::from_str(&data);
    //             }
    //         }
    //     }
        Ok(None)
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod desktop_persistence {
    use std::fs::File;
    use std::io::{Read, Write};
    use crate::model::Model;

    const PATH: &str = "./models";

    pub fn save_model(model: &Model) -> std::io::Result<()> {
        let file_o = rfd::FileDialog::new()
            .set_title("Save Model")
            .add_filter("DEMOSim files (*.dms)", &["dms"])
            .set_directory(PATH)
            .save_file();
        match file_o {
            None => Ok(()),
            Some(file) => {
                let ron = ron::ser::to_string_pretty(model, ron::ser::PrettyConfig::default()).unwrap(); // Serialize to RON format
                let mut file = File::create(file)?;
                file.write_all(ron.as_bytes())?;
                Ok(())
            },
        }
    }

    pub fn load_model() -> std::io::Result<Option<Model>> {
        let file_o = rfd::FileDialog::new()
            .set_title("Load Model")
            .add_filter("DEMOSim files (*.dms)", &["dms"])
            .set_directory(PATH)
            .pick_file();
        match file_o {
            None => Ok(None),
            Some(file) => {
                let mut file = File::open(file)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let model: Model = ron::from_str(&contents).unwrap();
                Ok(Some(model))
            },
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use self::web_persistence::*;

#[cfg(not(target_arch = "wasm32"))]
pub use self::desktop_persistence::*;
