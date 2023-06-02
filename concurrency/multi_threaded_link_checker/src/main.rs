use reqwest::blocking::{get};
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;
use std::sync::mpsc::{Sender, self};
use std::thread;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

fn extract_links(url: Url, tx: Sender<Url>) {
    let response = get(url).unwrap();
    let base_url = response.url().to_owned();
    let document = response.text().unwrap();
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(url) => _ = tx.send(url),
                Err(err) => {
                    println!("On {base_url}: could not parse {href:?}: {err} (ignored)",);
                }
            }
        }
    }
}

fn main() {
    let page_limit = 200;
    let mut page_count = 0;
    let (tx, rx) = mpsc::channel();

    let start_url = Url::parse("https://www.google.org").unwrap();
    _ = extract_links(start_url, tx.clone());

    for url in rx.iter() {
        page_count+=1;
        if page_limit < page_count {
            return()
        }

        let tx = tx.clone();
        thread::spawn(move || {
            println!("{url}");
            _ = extract_links(url, tx);
        });
    }
}