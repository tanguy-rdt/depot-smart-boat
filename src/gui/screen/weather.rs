use eframe::egui;
use poll_promise::Promise;
use serde_json::{Value};
use chrono::prelude::*;
use egui_extras::{Size, StripBuilder};
use eframe::egui::{Frame, Image, Rect, Vec2, Ui};


use crate::gui::screen::http_tools::Resource;
use crate::gui::screen::http_tools;

#[derive(Clone)]
#[derive(Default)]
struct OwmData{
    date: String,
    time: String,
    temperature: (f64, f64, f64, f64),
    pressure: i64,
    humidity: i64,
    weather_description: String, 
    wind_speed: f64,
    wind_direction: String,
    cloudiness: i64,
    icon: String,
}

pub struct Weather{
    temperature: f32,
    humidity: f32,
    pressure: f32,
    predictions: Vec<OwmData>,
    promise: Option<Promise<ehttp::Result<Resource>>>,
    selected_weather_index: usize,
    color_label: egui::Color32,
}

impl Weather{
    pub fn new() -> Self {
        Self {
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
            predictions: Vec::new(),
            promise: Default::default(),
            selected_weather_index: 0,
            color_label: egui::Color32::WHITE,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui.visuals().dark_mode { self.color_label = egui::Color32::WHITE; }
        else { self.color_label = egui::Color32::BLACK; }

        self.get_predictions(ctx, ui);
        if self.predictions.len() > 0 {
            self.show_selected_weather(ui, self.selected_weather_index);
            ui.separator();
            self.show_preview_weather(ui);
        }
    }

    fn show_selected_weather(&self, ui: &mut egui::Ui, index: usize) {
        StripBuilder::new(ui)
        .size(Size::relative(0.33)) 
        .vertical(|mut strip| {
            strip.strip(|builder| {
                builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                    strip.cell(|ui| {
                        if index == 0 {
                            ui.colored_label(self.color_label, "Current weather measure");
                            ui.label(format!("Temperature: {:.2} C", self.temperature));
                            ui.label(format!("Humidity: {:.2} %", self.humidity));
                            ui.label(format!("Pressure: {:.2} Pa", self.pressure)); 
                            ui.separator();
                        }
                        ui.colored_label(self.color_label, format!("Predict weather for {}, {}", self.predictions[index].date, self.predictions[index].time));
                        let (temp, feel_like, min, max) = self.predictions[index].temperature;
                        ui.label(format!("Temperature: {} C - Feel like: {} C", temp, feel_like));
                        ui.label(format!("min: {} C - max {} C", min, max));
                        ui.label(format!("Pressure: {:.2} hPa", self.predictions[index].pressure)); 
                        ui.label(format!("Humidity: {:.2}%", self.predictions[index].humidity)); 
                        ui.label(format!("wind: {} km/h - {} ", self.predictions[index].wind_speed, self.predictions[index].wind_direction)); 


                    });
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.colored_label(self.color_label, format!("{}", self.predictions[index].weather_description)); 
                            let image = egui::Image::new(format!("https://openweathermap.org/img/wn/{}@4x.png", self.predictions[index].icon));
                            let size = Vec2::new(170.0, 170.0); // Taille des images
                            let rect = Rect::from_min_size(ui.min_rect().min + Vec2::new(70.0, 15.0), size);
                            image.paint_at(ui, rect);
                        });
                    });
                });
            });
        });



    }

    fn show_preview_weather(&mut self, ui: &mut egui::Ui) {
        let mut current_date = "".to_string();
        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                egui::Grid::new("previewLayout")
                .num_columns(8)
                .striped(true)

                .show(ui, |ui| {
                    for (index, prediction) in self.predictions.iter().enumerate() {
                        if current_date != prediction.date {
                            current_date = prediction.date.clone();
                            ui.end_row();
                            ui.horizontal(|ui| {
                                ui.separator();
                                ui.colored_label(self.color_label, format!("{}", prediction.date))
                            });
                            ui.end_row();
                        }
                        ui.add(tile(&mut self.selected_weather_index, prediction.clone(), index));

                    }
                });
            });
        
        

    }

    fn show_tile(&self, ui: &mut egui::Ui, predictions: &OwmData) {
        /*ui.vertical(|ui|{
            let image = egui::Image::new(format!("https://openweathermap.org/img/wn/{}@2x.png", predictions.icon));
            let size = Vec2::new(50.0, 50.0); // Taille des images
            let rect = Rect::from_min_size(ui.min_rect().min + Vec2::new(0.0, 0.0), size);
            image.paint_at(ui, rect);

            ui.allocate_space(size);

            ui.label(format!("{}", predictions.time));
            let (temp, _, _, _) = predictions.temperature;
            ui.label(format!("{} C", temp));

        });*/

    }




    fn get_predictions(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let url_mapbox: String = "https://api.openweathermap.org/data/2.5/forecast?lat=48.3903&lon=-4.4853&appid=dc64e1d625ed2147ec0b6913a814f81d&units=metric".to_string();
        
        if ui.button("text").clicked() {
            self.promise = http_tools::fetch_ressource(ctx, url_mapbox);
        }
        

        if let Some(promise) = &self.promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(resource) => {
                        let Resource {
                            response,
                            text,
                            image,
                        } = resource;

                        if let Some(text) = text {
                            let v: Value = serde_json::from_str(text.as_str()).expect("Unable to parse JSON");
                            let directions = vec!["N", "NE", "E", "SE", "S", "SW", "W", "NW", "N"];

                            if let Some(list) = v["list"].as_array() {
                                self.predictions.clear();
                                self.predictions.shrink_to_fit();
                                for entry in list {
                                    let datetime_str = entry["dt_txt"].as_str().expect("Date and time missing in JSON").to_string();
                                    let datetime = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
                                    .expect("Invalid date and time format");
                                    let date = datetime.format("%m/%d").to_string();
                                    let time = datetime.format("%Hh%M").to_string();
                                    let temp = entry["main"]["temp"].as_f64().unwrap_or(0.0);
                                    let feels_like = entry["main"]["feels_like"].as_f64().unwrap_or(0.0);
                                    let temp_min = entry["main"]["temp_min"].as_f64().unwrap_or(0.0);
                                    let temp_max = entry["main"]["temp_max"].as_f64().unwrap_or(0.0);
                                    let pressure = entry["main"]["pressure"].as_i64().unwrap_or(0);
                                    let humidity = entry["main"]["humidity"].as_i64().unwrap_or(0);
                                    let weather_description = entry["weather"][0]["description"]
                                        .as_str()
                                        .unwrap_or("No description")
                                        .to_string();
                                    let icon = entry["weather"][0]["icon"]
                                        .as_str()
                                        .unwrap_or("No icon")
                                        .to_string();
                                    let wind_speed = entry["wind"]["speed"].as_f64().unwrap_or(0.0);
                                    let wind_direction = entry["wind"]["deg"].as_i64().unwrap_or(0);
                                    let index = ((wind_direction as f64 + 22.5) / 45.0).floor() as usize % directions.len();
                                    let wind_direction = directions[index].to_string();

                                    let cloudiness = entry["clouds"]["all"].as_i64().unwrap_or(0);

                                    self.predictions.push(OwmData {
                                        date: date,
                                        time: time,
                                        temperature: (temp, feels_like, temp_min, temp_max),
                                        pressure: pressure,
                                        humidity: humidity,
                                        weather_description: weather_description, 
                                        wind_speed: wind_speed,
                                        wind_direction: wind_direction,
                                        cloudiness: cloudiness,
                                        icon: icon,
                                    });
                                }
                            }
                        } 
                    }
                    Err(error) => {

                    }
                }
            } else {
                ui.spinner();
            }
        }
    }


    pub fn set_temperature(&mut self, value: f32){
        self.temperature = value;
    }

    pub fn set_humidity(&mut self, value: f32){
        self.humidity = value;
    }

    pub fn set_pressure(&mut self, value: f32){
        self.pressure = value;
    }
}

fn tile(on: &mut usize, prediction: OwmData, index: usize) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| tile_ui(ui, on, prediction, index)
}  

fn tile_ui(ui: &mut egui::Ui, on: &mut usize, prediction: OwmData, index: usize) -> egui::Response {
    let tile_size = egui::vec2(ui.spacing().interact_size.x * 1.7, ui.spacing().interact_size.y * 5.0);
    let (tile_rect, mut response) = ui.allocate_exact_size(tile_size, egui::Sense::click());
    let visuals = ui.style().interact_selectable(&response, false);

    if response.clicked() {
        *on = index;
        response.mark_changed();
    }
    if response.hovered() {
        ui.painter()
            .rect(tile_rect, 5.0, visuals.bg_fill, visuals.bg_stroke);
    }


    let image = egui::Image::new(format!("https://openweathermap.org/img/wn/{}@4x.png", prediction.icon));
    let size = Vec2::new(40.0, 40.0);
    let image_rect = egui::Rect::from_min_size(
        egui::pos2(tile_rect.min.x+15.0, tile_rect.min.y), 
        size, 
    );
    image.paint_at(ui, image_rect);
    
    let mut color_label = egui::Color32::BLACK;
    if ui.visuals().dark_mode { color_label = egui::Color32::WHITE; }
    else { color_label = egui::Color32::BLACK; }
    ui.painter().text(
        egui::pos2(tile_rect.min.x + ((tile_rect.max.x - tile_rect.min.x)/2.0), tile_rect.min.y + 45.0),
        egui::Align2::CENTER_CENTER,
        format!("{}", prediction.time),
        egui::FontId::default(),
        color_label
    );

    ui.painter().text(
        egui::pos2(tile_rect.min.x + ((tile_rect.max.x - tile_rect.min.x)/2.0), tile_rect.min.y + 60.0),
        egui::Align2::CENTER_CENTER, 
        format!("{} km/h", prediction.wind_speed),
        egui::FontId::new(12.0, egui::FontFamily::Proportional),
        ui.style().visuals.text_color()
    );

    ui.painter().text(
        egui::pos2(tile_rect.min.x + ((tile_rect.max.x - tile_rect.min.x)/2.0), tile_rect.min.y + 75.0),
        egui::Align2::CENTER_CENTER, 
        format!("{}",prediction.wind_direction),
        egui::FontId::new(12.0, egui::FontFamily::Proportional),
        ui.style().visuals.text_color()
    );



    response
}
