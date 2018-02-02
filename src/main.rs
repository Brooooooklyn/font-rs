extern crate font_rs;
extern crate image;
extern crate raster;

use std::io::Read;
use std::fs::File;
use std::path::Path;

use font_rs::font::{parse};

fn main() {
    let f = & File::open(Path::new("./test.woff")).unwrap();
    let data = & *f.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
    let r = parse(data).unwrap();
    let d = r.render_glyph(2, 400).unwrap();
    print!("{:?}", d);
}
