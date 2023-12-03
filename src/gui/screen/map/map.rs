use crate::gui::screen::map::InteractiveMap;
use crate::gui::menu::MenuSelection;
use crate::gui::screen::map::http_tools::Resource;
use crate::gui::screen::map::http_tools;

use std::time::{Duration, Instant};
use eframe::egui;
use poll_promise::Promise;

pub struct Map {
    interactive_map: InteractiveMap,
    last_url_mapbox: String,
    last_url_owm: String,
    promise_mapbox: Option<Promise<ehttp::Result<Resource>>>,
    promise_owm: Option<Promise<ehttp::Result<Resource>>>,
    last_fetch: Instant,
}

impl Map {
    pub fn new(egui_ctx: egui::Context) -> Self{
        Self {
            interactive_map: InteractiveMap::new(egui_ctx),
            last_url_mapbox: Default::default(),
            last_url_owm: Default::default(),
            promise_mapbox: Default::default(),
            promise_owm: Default::default(),
            last_fetch: Instant::now() - Duration::from_secs(60*5),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, menu_choice: &MenuSelection){
        match menu_choice {
            MenuSelection::MAP_CLASSIC => self.interactive_map.show(ui),
            MenuSelection::MAP_CLOUDS => self.show_map(ctx, ui, "outdoors-v12", "clouds_new"),
            MenuSelection::MAP_PRECIPITATION => self.show_map(ctx, ui, "light-v11", "precipitation_new"),
            MenuSelection::MAP_SEA_LEVEL_PRESSURE => self.show_map(ctx, ui, "outdoors-v12", "pressure_new"),
            MenuSelection::MAP_WIND_SPEED => self.show_map(ctx, ui, "outdoors-v12", "wind_new"),
            MenuSelection::MAP_TEMPERATURE => self.show_map(ctx, ui, "outdoors-v12", "temp_new"),
            _ => (),
        };
    }

    fn show_map(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, style_map: &str, style_weather: &str){
        let url_mapbox: String = format!("https://api.mapbox.com/styles/v1/mapbox/{}/tiles/0/0/0@2x?access_token=pk.eyJ1IjoidHJkdCIsImEiOiJjbHBwbGo3MG8wenIyMnJsZW1jY2dlaXkxIn0.w-MMac3G_ww9md68rpTugg", style_map);
        let url_owm: String = format!("https://tile.openweathermap.org/map/{}/0/0/0.png?appid=dc64e1d625ed2147ec0b6913a814f81d", style_weather);
        
        let now = Instant::now();
        if now.duration_since(self.last_fetch) > Duration::from_secs(60*5) ||
            self.last_url_mapbox != url_mapbox || self.last_url_owm != url_owm {
                
            self.last_url_mapbox = url_mapbox.clone();
            self.last_url_owm = url_owm.clone();

            self.last_fetch = now;
            self.promise_mapbox = http_tools::fetch_image(ctx, url_mapbox);
            self.promise_owm = http_tools::fetch_image(ctx, url_owm);
        }

        http_tools::get_image(&self.promise_mapbox, ui);
        http_tools::get_image(&self.promise_owm, ui);
    }
}