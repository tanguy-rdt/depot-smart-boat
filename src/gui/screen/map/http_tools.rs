use eframe::egui;
use eframe::egui::{Image, Rect, Vec2, Ui};
use poll_promise::Promise;

pub struct Resource {
    response: ehttp::Response,
    text: Option<String>,
    image: Option<Image<'static>>,
}

impl Resource {
    fn from_response(ctx: &egui::Context, response: ehttp::Response) -> Self {
        let content_type = response.content_type().unwrap_or_default();
        if content_type.starts_with("image/") {
            ctx.include_bytes(response.url.clone(), response.bytes.clone());
            let image = Image::from_uri(response.url.clone());

            Self {
                response,
                text: None,
                image: Some(image),
            }
        } else {
            let text = response.text();
            let text = text.map(|text| text.to_owned());

            Self {
                response,
                text,
                image: None,
            }
        }
    }
}

pub fn fetch_image(ctx: &egui::Context, url: String) -> Option<Promise<Result<Resource, String>>>{
    let prev_url = url.clone();
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get(url);
    ehttp::fetch(request, move |response| {
        ctx.forget_image(&prev_url);
        ctx.request_repaint(); 
        let resource = response.map(|response| Resource::from_response(&ctx, response));
        sender.send(resource);
    });
    Some(promise)
}

pub fn get_image(promise: &Option<Promise<Result<Resource, String>>>, ui: &mut Ui, ctx: &egui::Context) {
    if let Some(promise) = promise {
        if let Some(result) = promise.ready() {
            match result {
                Ok(resource) => {
                    ui_resource(ui, resource, ctx);
                }
                Err(error) => {
                    // This should only happen if the fetch API isn't available or something similar.
                    ui.colored_label(
                        ui.visuals().error_fg_color,
                        if error.is_empty() { "Error" } else { error },
                    );
                }
            }
        } else {
            ui.spinner();
        }
    }
}


fn ui_resource(ui: &mut egui::Ui, resource: &Resource, ctx: &egui::Context) {
    let Resource {
        response,
        text,
        image,
    } = resource;

    if let Some(image) = image {
        let image = image.clone();

        let available_rect = ctx.available_rect();
        let width = available_rect.width() - 170.0;
        let height = available_rect.height();

        let size = Vec2::new(width, height); // Taille des images
        let rect = Rect::from_min_size(ui.min_rect().min + Vec2::new(0.0, 0.0), size);
        image.paint_at(ui, rect);

    } else if let Some(text) = &text {
        selectable_text(ui, text);
    } else {
        ui.monospace("[binary]");
    }
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) {
    ui.add(
        egui::TextEdit::multiline(&mut text)
            .desired_width(f32::INFINITY)
            .font(egui::TextStyle::Monospace),
    );
}