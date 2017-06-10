// this allows us to use macros from the crate
#[macro_use]
// brings the clap crate in in the global scope of 'clap'
// and local scope of 'clap'
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate hyper;

// creates a module for scoping
mod errors {
    // executes the error_chain! macro
    error_chain!{
        foreign_links {
            // hyper isn't in the local scope of this module so
            // we refer to it in the global scope inside the macro
            // in this case we're creating a type 'Hyper' that contains
            // a hyper::Error
            Hyper(::hyper::Error);
        }
    }
}

// brings Arg and App from clap into the local scope
use clap::{Arg, App};
// brings all publicly exported types from the errors module
// above into scope. In this example we are using the custom
// Result type.
use errors::*;

// macro provided by error-chain to improve ergonomics around
// main(). This allows our function run() to be called by the real
// main() provided by this macro and prints the errors returned by
// run() to stderr.
quick_main!(run);

// the entry point to our application
fn run() -> Result<()> {
    // defines a new Clap based arg parser and a pretty name for the app
    let m = App::new("Rust Example")
        // provides us with a -V / --version that matches Cargo.toml
        .version(crate_version!())
        // creates an argument we'll reference as "METHOD"
        .arg(Arg::with_name("METHOD")
                 // allows it to be called as -m
                 .short("m")
                 // allows it to be called as --method
                 .long("method")
                 // show the field name as METHOD in help
                 .value_name("METHOD")
                 // we take a value named above
                 .takes_value(true)
                 // this field is required to be passed in
                 .required(true)
                 // only these values are valid
                 .possible_values(&["get", "post"])
                 // despite it being required default it so the user
                 // does not have to provide one
                 .default_value("get")
                 // help text
                 .help("HTTP method to use"))
        // create another argument referenced as URL that is positional in the first position
        .arg(Arg::with_name("URL").required(true).index(1).help("URL to show headers for"))
        // reads the args passed into main() and validate them or
        // report failure
        .get_matches();

    // reads the value of the positional argument
    // based on above we know it must be set so its ok to unwrap()
    let url = m.value_of("URL").unwrap();

    // create and initialize a new client
    let client = hyper::Client::new();
    // create and initialize a new request based on the METHOD
    let req = match m.value_of("METHOD") {
        // if there was a value and it contains "get" create a GET request
        Some("get") => client.get(url),
        // if there was a value and it contains "post" create a POST request
        Some("post") => client.post(url),
        // if there was a value and its any other value, panic!()
        Some(_) => panic!("this cannot happen"),
        // no value was provided for this field
        None => panic!("again, this cannot happen"),
    };

    // send the request and block until we get a response
    // ? is syntactic sugar for a Result give me the Ok value
    // and if its an Error return out of this function with that Error.
    let response = req.send()?;
    // macro to print a line to stdout
    println!("{} {}", response.version, response.status);
    // iterate over each header in the list of headers
    for header in response.headers.iter() {
        // macro to print to stdout without a trailing newline
        print!("{:?}", header);
    }

    // we must return a Result type so return Ok with unit
    Ok(())
}
