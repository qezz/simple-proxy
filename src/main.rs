extern crate hyper;

use hyper::{Client, Server};
use hyper::service::service_fn;
use hyper::rt::{self, Future};
use hyper_tls::HttpsConnector;

#[macro_use]
extern crate slog;
use slog::Drain;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;
extern crate slog_async;

use slog_envlogger;

#[macro_use]
extern crate log;

extern crate structopt;
use structopt::StructOpt;

use std::fs;
use std::path;


#[derive(Debug, StructOpt)]
#[structopt(name = "Data conventer", about = "Aptus geodata convertor")]
struct Opt {
    #[structopt(long = "log-to-file")]
    log_to_file: bool,

    #[structopt(short = "i", long = "in-addr")]
    in_addr: String,

    #[structopt(short = "o", long = "out-addr")]
    out_addr: String,
}


fn new_logger(opts: &Opt) -> slog::Logger {
    if opts.log_to_file {
        let log_path = "proxy.log";
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_path)
            .unwrap();

        let decorator = slog_term::PlainSyncDecorator::new(file);
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        // let drain = slog_envlogger::new(drain);
        let drain = slog_async::Async::new(drain).build().fuse();
        slog::Logger::root(drain.fuse(), o!())
    } else {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::CompactFormat::new(decorator).build();
        let drain = slog_envlogger::new(drain);
        let drain = std::sync::Mutex::new(drain).fuse();
        slog::Logger::root(drain, o!())
    }
}

fn main() {
    let opts = Opt::from_args();

    let logger = new_logger(&opts);
    let _guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();

    let in_addr = opts.in_addr.parse().expect("Unable to parse listener address");
    let out_addr = opts.out_addr;

    let https = HttpsConnector::new(4).unwrap();
    let client_main = Client::builder()
        .build::<_, hyper::Body>(https);

    let out_addr_clone = out_addr.clone();

    let new_service = move || {
        let client = client_main.clone();
        let out_addr_clone = out_addr_clone.clone();

        service_fn(move |req| {
            let uri_string = format!("{}/{}",
                                     out_addr_clone,
                                     req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""));
            info!("Trying {}", uri_string);

            client
                .get(uri_string.parse().unwrap())
        })
    };

    let server = Server::bind(&in_addr)
        .serve(new_service)
        .map_err(|e| error!("server error: {}", e));

    info!("Listening on {}", in_addr);
    info!("Proxying on {}", out_addr);

    rt::run(server);
}
