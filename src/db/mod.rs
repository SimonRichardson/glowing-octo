use iron::typemap::Key;
use rustless::{self, Extensible};

use mongodb::db::Database;

pub struct AppDb;

impl Key for AppDb { type Value = Database; }

pub trait DatabaseExt: rustless::Extensible {
    fn db(&self) -> &Database;
}
impl DatabaseExt for rustless::Application {
    fn db(&self) -> &Database {
        self.ext().get::<AppDb>().unwrap()
    }
}