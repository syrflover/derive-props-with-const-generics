#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

#[derive(Props)]
struct HelloProps<'a, const N: usize, const M: usize, const J: usize, const L: usize> {
    x: &'a str,
    xs: [u8; N],
    ys: [u8; M],
    js: [u8; J],
    ls: [u8; L],
}

fn Hello<'a, const N: usize, const M: usize, const J: usize, const L: usize>(
    cx: Scope<'a, HelloProps<'a, N, M, J, L>>,
) -> Element {
    let HelloProps { x, xs, ys, js, ls } = cx.props;

    cx.render(rsx! {
        div {
            span {
                "{x}"
            },
            br {},
            span {
                "{String::from_utf8(xs.to_vec()).unwrap()}"
            },
            br {},
            span {
                "{String::from_utf8(ys.to_vec()).unwrap()}"
            },
            br {},
            span {
                "{String::from_utf8(js.to_vec()).unwrap()}"
            },
            br {},
            span {
                "{String::from_utf8(ls.to_vec()).unwrap()}"
            }
        }
    })
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." },
            br {},
            Hello {
                x: "world",
                xs: [119, 111, 114, 108, 100, 32, 120, 115],
                ys: [119, 111, 114, 108, 100, 32, 121, 115, 33],
                js: [119, 111, 114, 108, 100, 32, 110, 115, 33, 33],
                ls: [119, 111, 114, 108, 100, 32, 109, 115, 33, 33, 33],
            }
        }
    ))
}
