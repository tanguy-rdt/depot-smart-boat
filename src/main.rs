mod voice_assistant;
mod model;
mod boat_control;
mod gui;
mod msgq;

use crate::voice_assistant::VoiceAssistant;
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
    model.init_model();
    let picovoice = VoiceAssistant::new(input_audio_path, keyword_path, context_path, access_key, Arc::clone(&msgq.tx_model));

    thread::spawn(move || {
        picovoice.start();
    });

    thread::spawn(move || {
        loop {
            match msgq.rx_model.lock().unwrap().try_recv() {
                Ok((var, val)) => {
                    model.treat_action(var.as_str(), val);
                }
                Err(_) => (),
            }
            model.get_temperature();
            model.get_humidity();
            model.get_pressure();
            model.get_boat_direction_degree();
            model.get_wind_direction_degree();
            thread::sleep(Duration::from_millis(500));
        }

    });

    let _ = eframe::run_native(
        "",
        Default::default(),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(Gui::new(msgq.rx_gui,msgq.tx_model, cc.egui_ctx.clone()))
        }),
    );

}