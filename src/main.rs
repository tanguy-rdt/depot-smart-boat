mod boat_control;

use crate::boat_control::BoatControler;

use std::{path::PathBuf, env, thread, time::Duration};
use std::sync::{mpsc, Arc, Mutex};

fn main(){
    let mut boat = BoatControler::new();
    boat.init();

    loop {
        println!("temperature: {:.2} C", boat.get_temperature());
        println!("pressure: {:.2} Pa", boat.get_pressure());
        println!("humidity: {:.2} %", boat.get_humidity());

        let duration = Duration::from_secs(2);
        thread::sleep(duration);
    }
}