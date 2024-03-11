use std::{path::PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use picovoice::{rhino::RhinoInference, PicovoiceBuilder};
use pv_recorder::PvRecorderBuilder;
use std::sync::atomic::{AtomicBool, Ordering};

const PPN_MODEL_PATH: &str = "./ressources/porcupine_params_fr.pv";
const RHN_MODEL_PATH: &str = "./ressources/rhino_params_fr.pv";

const HEADWIND: f32 = 0.5;
const CLOSEWIND_BABORD: f32 = 0.375;
const CROSSWIND_BABORD: f32 = 0.250;
const BEAMWIND_BABORD: f32 = 0.125;
const DOWNWIND: f32 = 0.0;
const BEAMWIND_TRIBORD: f32 = 0.875;
const CROSSWIND_TRIBORD: f32 = 0.750;
const CLOSEWIND_TRIBORD: f32 = 0.625;

static LISTENING: AtomicBool = AtomicBool::new(false);

pub struct VoiceAssistant {
    input_audio: PathBuf, 
    keyword_path: &'static str, 
    context_path: &'static str, 
    access_key: String,
    ppn_model_path: &'static str,
    rhn_model_path: &'static str,
    msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>,
}

impl VoiceAssistant {
    pub fn new(input_audio: PathBuf, keyword_path: &'static str, context_path: &'static str, access_key: String, msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>) -> Self {
        Self {
            input_audio,
            keyword_path,
            context_path,
            access_key,
            ppn_model_path: PPN_MODEL_PATH,
            rhn_model_path: RHN_MODEL_PATH,
            msgq_tx,
        }
    }

    pub fn start(&self){
        let audio_device_index = 0;
        let wake_word_callback = ||  println!("Wake word detected, awaiting instruction...");
        let inference_callback = |inference: RhinoInference| {
            if inference.is_understood {
                &self.treat_inference(inference);
            }
        };

        let mut picovoice_builder = PicovoiceBuilder::new(
            &self.access_key,
            &self.keyword_path,
            wake_word_callback,
            &self.context_path,
            inference_callback,
        );

        picovoice_builder = picovoice_builder.porcupine_model_path(&self.ppn_model_path);
        picovoice_builder = picovoice_builder.rhino_model_path(&self.rhn_model_path);

        let mut picovoice = picovoice_builder
                                                                .init()
                                                                .expect("Failed to create Picovoice");

        let recorder = PvRecorderBuilder::new(picovoice.frame_length() as i32)
            .device_index(audio_device_index)
            .init()
            .expect("Failed to initialize pvrecorder");
        recorder.start().expect("Failed to start audio recording");

        LISTENING.store(true, Ordering::SeqCst);
        ctrlc::set_handler(|| {
            LISTENING.store(false, Ordering::SeqCst);
        })
        .expect("Unable to setup signal handler");
    
        println!("Listening for commands...");

        while LISTENING.load(Ordering::SeqCst) {
            let frame = recorder.read().expect("Failed to read audio frame");
    
            picovoice.process(&frame).unwrap();
        }

        println!("\nStopping...");
        recorder.stop().expect("Failed to stop audio recording");
    }

    fn send_action(&self, value: f32) {
        self.msgq_tx
        .lock()
        .unwrap()
        .send(("mainsail_angle".to_string(), value))
        .unwrap();

        self.msgq_tx
        .lock()
        .unwrap()
        .send(("jib_angle".to_string(), value))
        .unwrap();
    }


    fn treat_inference(&self, inference: RhinoInference){
        match inference.intent.as_deref() {
            Some("direction_tribord") => self.send_action(1.0),
            Some("direction_babord") => self.send_action(0.0),
            Some("vent_près_tribord") => self.send_action(CLOSEWIND_TRIBORD),
            Some("vent_près_babord") => self.send_action(CLOSEWIND_BABORD),
            Some("vent_face") => self.send_action(HEADWIND),
            Some("vent_arrière") => self.send_action(DOWNWIND),
            Some("vent_largue_tribord") => self.send_action(BEAMWIND_TRIBORD),
            Some("vent_largue_babord") => self.send_action(BEAMWIND_BABORD),
            Some("vent_travers_babord") => self.send_action(CROSSWIND_BABORD),
            Some("vent_travers_tribord") => self.send_action(CROSSWIND_TRIBORD),
            _ => println!("Unknown intent: {}", inference.intent.unwrap()),
            
        };
    }
}


