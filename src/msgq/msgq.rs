use std::sync::{mpsc, Arc, Mutex};

pub struct MsgQ {
    pub tx_gui: Arc<Mutex<mpsc::Sender<(String, f32)>>>,
    pub rx_gui: Arc<Mutex<mpsc::Receiver<(String, f32)>>>,
    pub tx_model:  Arc<Mutex<mpsc::Sender<(String, f32)>>>,
    pub rx_model:  Arc<Mutex<mpsc::Receiver<(String, f32)>>>,
}

impl MsgQ {
    pub fn new() -> Self {
        let (gui_sender, gui_receiver) = mpsc::channel();
        let (model_sender, model_receiver) = mpsc::channel();

        Self {
            tx_gui: Arc::new(Mutex::new(gui_sender)),
            rx_gui: Arc::new(Mutex::new(gui_receiver)),
            tx_model:  Arc::new(Mutex::new(model_sender)),
            rx_model:  Arc::new(Mutex::new(model_receiver)),
        }
    }
}