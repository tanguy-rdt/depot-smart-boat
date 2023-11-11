mod gpio_manager;
mod picovoice_manager;

use crate::gpio_manager::gpio_itf::GpioItf;
use crate::gpio_manager::Gpio;
use crate::picovoice_manager::Picovoice;

use std::{path::PathBuf, env, thread, time::Duration};

fn main(){
    let input_audio_path = PathBuf::from("./ressources/OkBateau_gauche.wav");
    
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

    let gpio = Gpio::new();
    let picovoice = Picovoice::new(input_audio_path, keyword_path, context_path, access_key);

    gpio.init_gpio();
    picovoice.start()
}