use crate::gui::screen::map::InteractiveMap;
use crate::gui::menu::MenuSelection;
use crate::gui::screen::http_tools::Resource;
use crate::gui::screen::http_tools;

use std::time::{Duration, Instant};
use eframe::egui;
use poll_promise::Promise;
use eframe::egui::{Image, Rect, Vec2, Ui};

const OWM_API_TOKEN: &str = "fb4e1347c0ed70f5a3a62f9827e26855";
const MAPBOX_API_TOKEN: &str = "pk.eyJ1IjoidHJkdCIsImEiOiJjbHBwbGo3MG8wenIyMnJsZW1jY2dlaXkxIn0.w-MMac3G_ww9md68rpTugg";

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
        let url_mapbox: String = format!("https://api.mapbox.com/styles/v1/mapbox/{}/tiles/0/0/0@2x?access_token={}", style_map, MAPBOX_API_TOKEN);
        let url_owm: String = format!("https://tile.openweathermap.org/map/{}/0/0/0.png?appid={}", style_weather, OWM_API_TOKEN);
        
        let now = Instant::now();
        if now.duration_since(self.last_fetch) > Duration::from_secs(60*5) ||
            self.last_url_mapbox != url_mapbox || self.last_url_owm != url_owm {
                
            self.last_url_mapbox = url_mapbox.clone();
            self.last_url_owm = url_owm.clone();

            self.last_fetch = now;
            self.promise_mapbox = http_tools::fetch_ressource(ctx, url_mapbox);
            self.promise_owm = http_tools::fetch_ressource(ctx, url_owm);
        }

        self.show_image(&self.promise_mapbox, ui, ctx);
        self.show_image(&self.promise_owm, ui, ctx);
    }

    fn show_image(&self, promise: &Option<Promise<Result<Resource, String>>>, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(promise) = promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(resource) => {
                        let Resource {
                            response,
                            text,
                            image,
                        } = resource;

                        if let Some(image) = image {
                            let image = image.clone();
                    
                            let available_rect = ctx.available_rect();
                            let width = available_rect.width() - 165.0;
                            let height = available_rect.height() - 41.0;
                    
                            let size = Vec2::new(width, height); // Taille des images
                            let rect = Rect::from_min_size(ui.min_rect().min + Vec2::new(0.0, 0.0), size);
                            image.paint_at(ui, rect);
                    
                        } else {
                            ui.image(egui::include_image!("../img/error-web.png"));   
                        }
                    }
                    Err(error) => {
                        ui.image(egui::include_image!("../img/error-web.png"));   
                    }
                }
            } else {
                ui.spinner();
            }
        }
    }
}