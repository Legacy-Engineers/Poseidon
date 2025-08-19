mod audio_player;
use audio_player::AudioPlayer;

use eframe::egui;

struct App {
    audio: AudioPlayer,
    is_playing: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            audio: AudioPlayer::new(),
            is_playing: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let audio_file =
            "/home/borje/Downloads/Audio/Tyga_-_Taste__Official_Video__ft._Offset(256k).mp3";

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Poseidon 🎵");

            if ui.button("▶ Play").clicked() {
                if !self.is_playing {
                    self.audio.play_file(audio_file);
                    self.is_playing = true;
                }
            }

            if ui.button("⏹ Stop").clicked() {
                self.audio.stop();
                self.is_playing = false;
            }
        });

        // Reset state when finished
        if self.is_playing && self.audio.is_empty() {
            self.is_playing = false;
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Poseidon",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
    .unwrap();
}
