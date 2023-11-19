mod model;
mod boat_control;
mod gui;

use crate::model::Model;
use crate::gui::Gui;

use std::{path::PathBuf, env, thread, time::Duration};
use std::sync::{mpsc, Arc, Mutex};

fn main(){
    let (msgq_sender, msgq_receiver) = mpsc::channel();
    let msgq = Arc::new(Mutex::new(msgq_sender));

    let mut model = Model::new();
    let gui = Gui::new(Arc::clone(&msgq));
}