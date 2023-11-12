mod picovoice_manager;
mod model;
mod gpio_manager;
mod gui;

use crate::picovoice_manager::Picovoice;
use crate::model::Model;
use crate::gui::Gui;

use std::{path::PathBuf, env, thread, time::Duration};
use std::sync::{mpsc, Arc, Mutex};

fn main(){
    let input_audio_path = PathBuf::from("./ressources/audio.wav");
    
    #[cfg(target_os = "macos")]
    let keyword_path = "./ressources/Ok-Bateau_fr_mac_v3_0_0.ppn";
    #[cfg(target_os = "macos")]
    let context_path = "./ressources/smart-boat_fr_mac_v3_0_0.rhn";
    
    #[cfg(target_os = "linux")]
    let keyword_path = "./ressources/Ok-Bateau_fr_raspberry-pi_v3_0_0.ppn";
    #[cfg(target_os = "linux")]
    let context_path = "./ressources/smart-boat_fr_raspberry-pi_v3_0_0.rhn";

    let access_key: String = match env::var("PICOVOICE_ACCES_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: you need add your acces key 'export PICOVOICE_ACCES_KEY=\"...\"'");
            std::process::exit(1);
        }
    };

    let (sender, receiver) = mpsc::channel();
    let msgq = Arc::new(Mutex::new(sender));

    let mut model = Model::new();
    let gui = Gui::new(Arc::clone(&msgq));
    let picovoice = Picovoice::new(input_audio_path, keyword_path, context_path, access_key, Arc::clone(&msgq));

    thread::spawn(move || {
        picovoice.start();
    });

    //gui.set_mainsail_angle(1);

    loop {
        match receiver.recv() {
            Ok(action) => {
                model.treat_action(action.as_str());
                gui.update(&model);
            }
            Err(_) => break,
        }
    }
}