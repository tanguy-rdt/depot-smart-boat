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
    #[cfg(target_os = "macos")]
    let keyword_path = "./resources/Ok-Bateau_fr_mac_v3_0_0.ppn";
    #[cfg(target_os = "macos")]
    let context_path = "./resources/smart-boat_fr_mac_v3_0_0.rhn";
    
    #[cfg(target_os = "linux")]
    let keyword_path = "./resources/Ok-Bateau_fr_raspberry-pi_v3_0_0.ppn";
    #[cfg(target_os = "linux")]
    let context_path = "./resources/smart-boat_fr_raspberry-pi_v3_0_0.rhn";

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

    
    let picovoice = VoiceAssistant::new(keyword_path, context_path, access_key, Arc::clone(&msgq.tx_model));

    thread::spawn(move || {
        picovoice.start();
    });

    let mut sail_automation_enabled = false;

    thread::spawn(move || {
        loop {
            match msgq.rx_model.lock().unwrap().try_recv() {
                Ok((var, val)) => {
                    if var == "automation" {
                        sail_automation_enabled = val == 1.0;
                    } 
                    model.treat_action(var.as_str(), val);
                }
                Err(_) => (),
            }
            model.get_temperature();
            model.get_humidity();
            model.get_pressure();
            model.get_boat_direction_degree();
            model.get_wind_direction_degree();
            model.get_deep();

            if sail_automation_enabled {
                model.treat_action("automation", 0.0)
            }

            thread::sleep(Duration::from_millis(500));
        }

    });

    let options = eframe::NativeOptions {
        resizable: false,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(Gui::new(msgq.rx_gui, msgq.tx_model, cc.egui_ctx.clone()))
        }),
    );

}