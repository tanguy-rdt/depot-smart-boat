use std::{path::PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use picovoice::{rhino::RhinoInference, PicovoiceBuilder};
use pv_recorder::PvRecorderBuilder;
use std::sync::atomic::{AtomicBool, Ordering};

const PPN_MODEL_PATH: &str = "./ressources/porcupine_params_fr.pv";
const RHN_MODEL_PATH: &str = "./ressources/rhino_params_fr.pv";

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


    fn treat_inference(&self, inference: RhinoInference){
        let mut action: String = String::new();

        match inference.intent.as_deref() {
            Some("direction_tribord") => action = "mainsail_angle".to_string(),
            Some("direction_babord") => action = "jib_angle".to_string(),
            _ => {
                println!("Unknown intent: {}", inference.intent.unwrap());
            }
        }

        if !action.is_empty() {
            println!("{action}");
            self.msgq_tx
            .lock()
            .unwrap()
            .send((action, 0.0))
            .unwrap();
        }
    }
}