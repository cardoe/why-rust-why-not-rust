#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate hyper;

mod errors {
    error_chain!{
        foreign_links {
            Hyper(::hyper::Error);
        }
    }
}

use clap::{Arg, App};
use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let m = App::new("Rust Example")
        .version(crate_version!())
        .arg(Arg::with_name("METHOD")
                 .short("m")
                 .long("method")
                 .value_name("METHOD")
                 .takes_value(true)
                 .required(true)
                 .possible_values(&["get", "post"])
                 .default_value("get")
                 .help("HTTP method to use"))
        .arg(Arg::with_name("URL").required(true).index(1).help("URL to show headers for"))
        .get_matches();

    // based on above we know it must be set so its ok to unwrap()
    let url = m.value_of("URL").unwrap();

    let client = hyper::Client::new();
    let req = match m.value_of("METHOD") {
        Some("get") => client.get(url),
        Some("post") => client.post(url),
        _ => unreachable!(),
    };

    let response = req.send()?;
    println!("{} {}", response.version, response.status);
    for header in response.headers.iter() {
        print!("{:?}", header);
    }

    Ok(())
}
