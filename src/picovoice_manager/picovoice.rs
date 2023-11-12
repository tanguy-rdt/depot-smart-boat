use std::{path::PathBuf, env, thread, time::Duration};
use std::sync::{mpsc, Arc, Mutex};
use picovoice::{rhino::RhinoInference, PicovoiceBuilder};
use itertools::Itertools;

const PPN_MODEL_PATH: &str = "./ressources/porcupine_params_fr.pv";
const RHN_MODEL_PATH: &str = "./ressources/rhino_params_fr.pv";

pub struct Picovoice {
    input_audio: PathBuf, 
    keyword_path: &'static str, 
    context_path: &'static str, 
    access_key: String,
    ppn_model_path: &'static str,
    rhn_model_path: &'static str,
    msgq: Arc<Mutex<mpsc::Sender<String>>>,
}

impl Picovoice {
    pub fn new(input_audio: PathBuf, keyword_path: &'static str, context_path: &'static str, access_key: String, msgq: Arc<Mutex<mpsc::Sender<String>>>) -> Self {
        Picovoice {
            input_audio,
            keyword_path,
            context_path,
            access_key,
            ppn_model_path: PPN_MODEL_PATH,
            rhn_model_path: RHN_MODEL_PATH,
            msgq,
        }
    }

    pub fn start(&self){
        let wake_word_callback = || ();
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

        
        let mut wav_reader = match hound::WavReader::open(&self.input_audio.clone()) {
            Ok(reader) => reader,
            Err(err) => panic!(
                "Failed to open .wav audio file {}: {}",
                &self.input_audio.display(),
                err
            ),
        };

        if wav_reader.spec().sample_rate != picovoice.sample_rate() {
            panic!(
                "Audio file should have the expected sample rate of {}, got {}",
                picovoice.sample_rate(),
                wav_reader.spec().sample_rate
            );
        }

        if wav_reader.spec().channels != 1u16 {
            panic!(
                "Audio file should have the expected number of channels 1, got {}",
                wav_reader.spec().channels
            );
        }

        if wav_reader.spec().bits_per_sample != 16u16
            || wav_reader.spec().sample_format != hound::SampleFormat::Int
        {
            panic!("WAV format should be in the signed 16 bit format",);
        }

        for frame in &wav_reader
            .samples()
            .chunks(picovoice.frame_length() as usize)
        {
            let frame: Vec<i16> = frame.map(|s| s.unwrap()).collect_vec();
            if frame.len() == picovoice.frame_length() as usize {
                picovoice.process(&frame).unwrap();
            }
        }
    }

    fn wake_word(&self){

    }

    fn treat_inference(&self, inference: RhinoInference){
        let mut action: String = String::new();

        match inference.intent.as_deref() {
            Some("direction_tribord") => action = "direction_tribord".to_string(),
            Some("direction_babord") => action = "direction_babord".to_string(),
            _ => {
                println!("Unknown intent: {}", inference.intent.unwrap());
            }
        }

        if !action.is_empty() {
            self.msgq
            .lock()
            .unwrap()
            .send(action)
            .unwrap();
        }
    }
}