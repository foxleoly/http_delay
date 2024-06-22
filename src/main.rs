use reqwest::blocking::Client;
use std::{env, ops::Not, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    // catch the args err
    if args.is_empty() {
        println!("Usage: http_delay https://www.bing.com");
        return;
    } else {
        println!("Req: {}", &args[1]);
    };
    let mut http_url = String::new();
    let url = args[1].as_str();
    if url.starts_with("http").not() {
        http_url = format!("http://{url}");
    } else {
        http_url = format!("{url}");
    };

    // init http client
    let http_clent = Client::new();
    let time_now = Instant::now();
    let http_res = http_clent.get(http_url).send();

    match http_res {
        Ok(response) => {
            println!("elapsed: {} ms", time_now.elapsed().as_millis());
            println!("code {}", response.status());
        }
        Err(error) => {
            println!("Error! {}", error);
        }
    }
}
