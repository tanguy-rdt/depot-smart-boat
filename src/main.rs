mod model;
mod boat_control;
mod gui;
mod msgq;

use crate::model::Model;
use crate::gui::Gui;
use crate::msgq::MsgQ;

use std::sync::{mpsc, Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
fn main(){
    let msgq = MsgQ::new();
    let mut model = Model::new(Arc::clone(&msgq.tx_gui));



    eframe::run_native(
        "",
        Default::default(),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(Gui::new(msgq.rx_gui,msgq.tx_model, cc.egui_ctx.clone()))
        }),
    );

}


#[cfg(target_arch = "wasm32")]
fn main() {

    let msgq = MsgQ::new();
    let mut model = Model::new(Arc::clone(&msgq.tx_gui));

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();


    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "smart-boat", 
                web_options, 
                Box::new(|cc| {
                    egui_extras::install_image_loaders(&cc.egui_ctx);
                    Box::new(Gui::new(msgq.rx_gui,msgq.tx_model, cc.egui_ctx.clone()))
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
