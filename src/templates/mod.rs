mod additional_builtins;
mod nix;

use additional_builtins::human_readable_size;
use nix::{name_from_store_path, strip_nix_store};

use std::str::from_utf8;

use axum::response::Html;
use lazy_static::lazy_static;
use rust_embed::Embed;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();

        tera.register_filter("human_readable_size", human_readable_size);
        tera.register_filter("name_from_store_path", name_from_store_path);
        tera.register_filter("strip_nix_store", strip_nix_store);

        let _res = tera.add_raw_templates(Template::iter().map(|file| {
            let raw_data = Template::get(&file).unwrap();
            let content = from_utf8(raw_data.data.as_ref()).unwrap();
            (file.to_string(), content.to_string())
        }));
        tera
    };
}

#[derive(Embed)]
#[folder = "templates/"]
struct Template;

pub fn render_template(page: &str, ctx: &Context) -> Html<String> {
    let render = format!("pages/{page}.tera");

    let rendered = TEMPLATES.render(&render, ctx).unwrap();

    Html(rendered)
}
