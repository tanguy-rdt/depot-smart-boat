mod model;
mod boat_control;
mod gui;

use crate::model::Model;
use crate::gui::Gui;
use crate::boat_control::BoatControler;

use std::{path::PathBuf, env, thread, time::Duration};
use std::sync::{mpsc, Arc, Mutex};

fn main(){
    let (msgq_sender, msgq_receiver) = mpsc::channel();
    let msgq = Arc::new(Mutex::new(msgq_sender));

    let mut model = Model::new();
    let gui = Gui::new(Arc::clone(&msgq));

    let mut boat = BoatControler::new();
    boat.init();
    //println!("temperature: {}", boat.get_temperature());
    println!("pressure: {}", boat.get_pressure());
    println!("humidity: {}", boat.get_humidity());
}