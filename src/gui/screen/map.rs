use walkers::{Tiles, Map, MapMemory, Position, providers::OpenStreetMap};
use eframe::egui::{Context, Align2, RichText, Ui, Window};

pub struct Osm {
    tiles: Tiles,
    map_memory: MapMemory,
    coord: (f64, f64),
}

impl Osm {
    pub fn new(egui_ctx: Context) -> Self {
        Self {
            tiles: Tiles::new(OpenStreetMap, egui_ctx),
            map_memory: MapMemory::default(),
            coord: (-4.4853,48.3903),
        }
    }

    pub fn get_map(&mut self) -> Map {
        let (x, y) = self.coord;

        Map::new(
            Some(&mut self.tiles),
            &mut self.map_memory,
            Position::from_lon_lat(x,y)
        )
    }

    pub fn zoom(&mut self, ui: &Ui) {
        Window::new("Map")
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .anchor(Align2::LEFT_BOTTOM, [20., -40.])
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("➕").heading()).clicked() {
                        let _ = self.map_memory.zoom_in();
                    }
    
                    if ui.button(RichText::new("➖").heading()).clicked() {
                        let _ = self.map_memory.zoom_out();
                    }
                });
            });
    }

    pub fn position(&mut self, ui: &Ui) {
        if let Some(position) = self.map_memory.detached() {
            Window::new("Center")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(Align2::RIGHT_BOTTOM, [-170., -40.])
                .show(ui.ctx(), |ui| {
                    ui.label(format!("{:.04} {:.04}", position.lon(), position.lat()));
                    if ui
                        .button(RichText::new("Go to my position").heading())
                        .clicked()
                    {
                        self.map_memory.follow_my_position();
                    }
                });
        }
        else {
            let (x, y) = self.coord;
            Window::new("Center")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(Align2::RIGHT_BOTTOM, [-170., -40.])
                .show(ui.ctx(), |ui| {
                    ui.label(format!("{:.04} {:.04}", x, y));
                });
        }
    }
}