use dioxus::prelude::*;
use dioxus_sdk::set_dir;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
use qrcode::render::svg;
use qrcode::QrCode;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    set_dir!();
    dioxus::launch(App);
}

fn generate_qr_svg(data: &str) -> Result<String, String> {
    QrCode::new(data)
        .map(|qr| {
            qr.render::<svg::Color<'_>>()
                .quiet_zone(false)
                .min_dimensions(220, 220)
                .max_dimensions(220, 220)
                .build()
        })
        .map_err(|e| e.to_string())
}

#[component]
fn App() -> Element {
    let mut input = use_synced_storage::<LocalStorage, String>("qr_txt".into(), || "".into());
    let mut qr_svg = use_synced_storage::<LocalStorage, Option<String>>("qr_svg".into(), || None);
    let mut error_msg =
        use_synced_storage::<LocalStorage, Option<String>>("qr_err".into(), || None);

    let input_val = input.read();
    let is_disabled = input_val.is_empty();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "container",
            input {
                r#type: "text",
                value: "{input_val}",
                placeholder: "Paste link here...",
                oninput: move |e| input.set(e.value()),

            }
            button {
                class: "primary",
                disabled: "{is_disabled}",
                onclick: move |_| {
                    if !input.read().is_empty() {
                        match generate_qr_svg(&input.read()) {
                            Ok(svg) => {
                                qr_svg.set(Some(svg));
                                error_msg.set(None);
                            }
                            Err(e) => {
                                qr_svg.set(None);
                                error_msg.set(Some(e));
                            }
                        }
                    }
                },
                "Generate QR Code"
            }

            // Error display
            if let Some(err) = error_msg.read().clone() {
                // div { class: "error-msg", "{err}" }
                div {class: "qr-box", dangerous_inner_html: err}
            } else {
                // div {class: "error-msg", ""}
                div {
                    class: if qr_svg.read().is_some() { "qr-box show" } else { "qr-box" },
                    dangerous_inner_html: qr_svg.read().clone().unwrap_or_default()
                }
            }


            if qr_svg.read().is_some() || error_msg.read().is_some() {
                    button {
                        class: "secondary",
                        onclick: move |_| {
                            input.set("".into());
                            qr_svg.set(None);
                            error_msg.set(None);
                        },
                        "Clear"
                    }
            }

        }
    }
}
