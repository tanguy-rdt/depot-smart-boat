mod picovoice_manager;
mod model;
mod boat_control;
mod gui;
mod msgq;

use crate::picovoice_manager::Picovoice;
use crate::model::Model;
use crate::gui::Gui;
use crate::msgq::MsgQ;

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

    let msgq = MsgQ::new();

    let mut model = Model::new(Arc::clone(&msgq.tx_gui));
    let picovoice = Picovoice::new(input_audio_path, keyword_path, context_path, access_key, Arc::clone(&msgq.tx_pv));

    thread::spawn(move || {
        picovoice.start();
    });

    thread::spawn(move || {
        loop {
            match msgq.rx_pv.lock().unwrap().try_recv() {
                Ok(action) => {
                    model.treat_action(action.as_str());
                }
                Err(_) => (),
            }
            model.get_temperature();
            model.get_humidity();
            model.get_pressure();
            thread::sleep(Duration::from_millis(500));
        }

    });

    eframe::run_native(
        "",
        Default::default(),
        Box::new(|cc| Box::new(Gui::new(msgq.rx_gui, cc.egui_ctx.clone()))),
    );

}