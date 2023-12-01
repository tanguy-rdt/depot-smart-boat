use std::sync::{mpsc, Arc, Mutex};

pub struct MsgQ {
    pub tx_gui: Arc<Mutex<mpsc::Sender<(String, f32)>>>,
    pub rx_gui: Arc<Mutex<mpsc::Receiver<(String, f32)>>>,
    pub tx_pv:  Arc<Mutex<mpsc::Sender<String>>>,
    pub rx_pv:  Arc<Mutex<mpsc::Receiver<String>>>,
}

impl MsgQ {
    pub fn new() -> Self {
        let (model_to_gui_sender, model_to_gui_receiver) = mpsc::channel();
        let (controller_to_model_sender, controller_to_model_receiver) = mpsc::channel();

        Self {
            tx_gui: Arc::new(Mutex::new(model_to_gui_sender)),
            rx_gui: Arc::new(Mutex::new(model_to_gui_receiver)),
            tx_pv:  Arc::new(Mutex::new(controller_to_model_sender)),
            rx_pv:  Arc::new(Mutex::new(controller_to_model_receiver)),
        }
    }
}