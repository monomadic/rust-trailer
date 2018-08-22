#![allow(unused_variables)]
#![allow(dead_code)]

use horrorshow::prelude::*;
use error::*;

use trailer::presenters::*;

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
            body(class="desktop-constrain padding-horizontal-m") {: Raw(content) }
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
    // let header = ::views::header()?;
    // let page_html = ::template::render(&format!("data/{}.templar", coin.symbol))?;

    let funds = funds.alts.into_iter().map(|asset| {
        format!("<tr>{}{}{}</tr>", td(&asset.asset.symbol), td(&usd_price(asset.value_in_btc)), td(&usd_price(asset.value_in_usd)))
    }).collect::<Vec<String>>().join("");

    Ok(table(&funds))

    // ::views::layout(
    //     "funds",
    //     html! {
    //         : Raw(header.clone());
    //         table(class="funds") {
    //             @ for asset in funds.alts.clone() {
    //                 div(class="asset") {
    //                     div(class="asset") {@ asset }
    //                 }
    //             }
    //         }
    //     }.to_string()
    // )

    // ::views::layout(
    //     "funds",
    //     div({
    //         funds.alts.into_iter().map(|asset| {
    //             div(&format!("{:?}", asset))
    //         }).collect::Vec<String>().join()
    //     })
    // )

}

fn table(content: &str) -> String { format!("<table>{}</table>", content) }
fn td(content: &str) -> String { format!("<td>{}</td>", content) }
fn div(content: &str) -> String { format!("<div>{}</div>", content) }
fn usd_price(content: f64) -> String { format!("${:.2}", content) }
