#![allow(unused_variables)]
#![allow(dead_code)]

use horrorshow::prelude::*;
use error::*;

use trailer::presenters::*;

pub mod position;
pub mod funds;
pub mod rsi;

pub fn layout(title: &str, content: String) -> Result<String, ServerError> {
    return Ok(html! {
        : ::horrorshow::helper::doctype::HTML;
        html {
            head {
                title {: title }

                meta(http-equiv="X-UA-Compatible", content="chrome=1");
                meta(name="viewport", content="initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=0");
                meta(name="apple-touch-fullscreen", content="yes");
                meta(name="apple-mobile-web-app-capable", content="yes");

                link(rel="stylesheet", type="text/css", href="/static/style.css?v=3", media="all");
                link(rel="stylesheet", href="https://fonts.googleapis.com/css?family=Roboto+Mono");
                link(rel="icon", type="image/png", href="/static/favicon.png");
            }
            body(class="desktop-constrain padding-horizontal-m", style="background-color: #111; color: white;") {: Raw(content) }
        }
    }.into_string()?)
}

pub fn header() -> Result<String, ServerError> {
    return Ok(html! {
        header {
            // : Raw(youtube_video("5PsQPpFgvu4"));
            a(href="/") { img(src="/static/logo.png", height="31px") }
            span(class="no-wrap") {: "crypto research reports" }
        }
    }.into_string()?)
}

pub fn funds(funds: FundsPresenter) -> Result<String, ServerError> {
    layout("funds", pre(&funds::text(funds)))
}

fn pre(content: &str) -> String { format!("<pre>{}</pre>", content) }
fn table(content: &str) -> String { format!("<table>{}</table>", content) }
fn td(content: &str) -> String { format!("<td>{}</td>", content) }
fn div(content: &str) -> String { format!("<div>{}</div>", content) }
fn usd_price(content: f64) -> String { format!("${:.2}", content) }
fn btc_price(content: f64) -> String { format!("{:.4} btc", content) }

pub fn display_fiat(num: f64) -> String {
    format!("${:.2}", num)
}

pub fn colored_number(num: f64, formatted_string: String) -> String {
    match num > 0.0 {
        true => format!("<span style='color:#0F0;'>{}</span>", formatted_string),
        false => format!("<span style='color:#F00;'>{}</span>", formatted_string),
    }
}

