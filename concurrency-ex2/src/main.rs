use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex, OnceLock};
use std::thread;

use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

const MAXIMUM_URL: i32 = 100;
static URL_COUNTER: OnceLock<Mutex<i32>> = OnceLock::new();

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

fn extract_links(response: Response) -> Result<Vec<Url>, Error> {
    let base_url = response.url().to_owned();
    let document = response.text()?;
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    // println!("base: {}", base_url);
    let mut valid_urls = Vec::new();
    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(url) => valid_urls.push(url),
                Err(err) => {
                    println!("On {base_url}: could not parse {href:?}: {err} (ignored)",);
                }
            }
        }
    }

    Ok(valid_urls)
}

fn extract_link_from_url(url: String, seen: &mut HashSet<String>) {
    if seen.contains(&url) {
        return;
    }
    let counter = URL_COUNTER.get_or_init(|| Mutex::new(0));
    if *counter.lock().unwrap() > MAXIMUM_URL {
        return;
    }

    println!("extracting >>> {}", url);
    seen.insert(url.to_owned());

    let (tx, rx): (Sender<Response>, Receiver<Response>) = mpsc::channel();
    let sender = tx.clone();
    thread::spawn(move || {
        let start_url = Url::parse(url.as_str()).unwrap();
        if let Ok(response) = get(start_url) {
            *counter.lock().unwrap() += 1;
            sender.send(response).unwrap();
        }
    });

    for response in rx.recv() {
        match extract_links(response) {
            Ok(links) => {
                for l in links {
                    extract_link_from_url(l.to_string(), seen);
                }
            }
            Err(err) => println!("Could not extract links: {err:#}"),
        }
    }
}

fn main() {
    let start_url = "https://www.google.org".to_string();
    let mut seen = HashSet::<String>::new();

    extract_link_from_url(start_url, &mut seen);
}
