extern crate url;
extern crate url_open;

use url::Url;
use url_open::UrlOpen;

fn main() {
    Url::parse("https://github.com/richardanaya/js-wasm").unwrap().open();
}