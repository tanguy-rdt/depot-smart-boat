use eframe::egui;
use eframe::egui::{Image, Rect, Vec2, Ui};
use poll_promise::Promise;

pub struct Resource {
    pub response: ehttp::Response,
    pub text: Option<String>,
    pub image: Option<Image<'static>>,
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

pub fn fetch_ressource(ctx: &egui::Context, url: String) -> Option<Promise<Result<Resource, String>>>{
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
