use crate::model::Model;

use std::sync::{mpsc, Arc, Mutex};

pub struct Gui{
    msgq: Arc<Mutex<mpsc::Sender<String>>>,
}

impl Gui {
    pub fn new(msgq: Arc<Mutex<mpsc::Sender<String>>>) -> Self {
        Gui { msgq }
    }

    pub fn set_mainsail_angle(&self, angle: i8){
        if angle == 1{
            self.msgq
            .lock()
            .unwrap()
            .send("direction_tribord".to_string())
            .unwrap();
        } else {
            self.msgq
            .lock()
            .unwrap()
            .send("direction_babord".to_string())
            .unwrap();
        }
    }

    pub fn update(&self, model: &Model){
        println!("*********************************");
        println!("         Affichage dans UI");
        println!("*********************************");

        println!("angle mainsail: {}", model.get_mainsail_angle());
        println!("angle foque: {}", model.get_foque_angle());
    }
}