use eframe::egui;
use egui_extras::{Size, StripBuilder};

pub struct Control {
    mainsail: bool
}

impl Control {
    pub fn new() -> Self {
        Self {
            mainsail: false,
        }
    }

    fn show_image(&mut self, ui:  &mut egui::Ui){
        match self.mainsail {
            true => {
                if ui.visuals().dark_mode {
                    ui.image(egui::include_image!(
                        "./img/dark_theme/boat_actif.png"
                    ));
                }
                else {
                    ui.image(egui::include_image!(
                        "./img/light_theme/boat_actif.png"
                    ));
                }

            },
            false => {
                if ui.visuals().dark_mode {
                    ui.image(egui::include_image!(
                        "./img/dark_theme/boat_inactif.png"
                    ));
                }
                else {
                    ui.image(egui::include_image!(
                        "./img/light_theme/boat_inactif.png"
                    ));
                }
            },
            _ => (),
        };
    }

    fn show_cmd(&mut self, ui:  &mut egui::Ui) {
        egui::Grid::new("TextLayoutDemo")
        .num_columns(2)
        .show(ui, |ui| {
            ui.add(toggle(&mut self.mainsail));
            ui.label("mainsail");
            ui.end_row();
        });
    }

    pub fn show(&mut self, ui:  &mut egui::Ui){
        StripBuilder::new(ui)
        .size(Size::relative(1.0)) // Diviser l'espace en deux colonnes, chaque colonne ayant la moitié de la largeur disponible
        .vertical(|mut strip| {
            strip.strip(|builder| {
                builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                    strip.cell(|ui| {
                        self.show_image(ui);
                    });
                    strip.cell(|ui| {
                        self.show_cmd(ui);
                    });
                });
            });
        });
    }
}

fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}  