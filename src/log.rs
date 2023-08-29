extern crate slog;
extern crate slog_async;
extern crate slog_json;
extern crate lazy_static;

use slog::{Drain, Logger, o};
use lazy_static::lazy_static;
use std::io;

lazy_static! {
    pub static ref LOGGER: Logger = {
        let decorator = io::stdout();
        let drain = slog_json::Json::new(decorator)
            .build()
            .fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        Logger::root(drain, o!())
    };
}