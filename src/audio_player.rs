use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub struct AudioPlayer {
    _stream_handle: OutputStream, // must be kept alive
    sink: Arc<Mutex<Sink>>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let _stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
        let mixer = _stream_handle.mixer();
        let sink = Sink::connect_new(&mixer);
        Self {
            _stream_handle,
            sink: Arc::new(Mutex::new(sink)),
        }
    }

    pub fn play_file(&self, file_path: &str) {
        let file = File::open(file_path).expect("Failed to open audio file");
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).expect("Failed to decode audio file");

        self.sink.lock().unwrap().append(source);
    }

    pub fn stop(&self) {
        self.sink.lock().unwrap().stop();
    }

    pub fn is_empty(&self) -> bool {
        self.sink.lock().unwrap().empty()
    }
}
