use lyon::path::default::{Path};

#[cfg(feature="sse")]
#[link(name = "accumulate")]
extern {
    fn accumulate_sse(src: *const f32, dst: *mut u8, n: u32);
}

#[derive(Debug)]
pub struct GlyphBitmap {
    pub width: usize,
    pub height: usize,
    pub left: i32,
    pub top: i32,
    pub a: Path,
}

impl GlyphBitmap {
    pub fn new (width: usize, height: usize, left: i32, top: i32, path: Path) -> GlyphBitmap {
        GlyphBitmap { width, height, left, top, a: path }
    }

    // #[cfg(feature="sse")]
    // pub fn get_bitmap(&self) -> Vec<u8> {
    //     let dst_size = self.width * self.height;
    //     let dst_cap = (dst_size + 3) & !3;
    //     let mut r: Vec<u8> = Vec::with_capacity(dst_cap);
    //     unsafe {
    //         accumulate_sse(self.a.as_ptr(), r.as_mut_ptr(), dst_cap as u32);
    //         r.set_len(dst_size);
    //     }
    //     r
    // }

    // #[cfg(not(feature="sse"))]
    // pub fn get_bitmap(&self) -> Vec<u8> {
    //     let mut acc = 0.0;
    //     (0..self.width * self.height).map(|i| {
    //     // This would translate really well to SIMD
    //         acc += self.a[i];
    //         let y = acc.abs();
    //         let y = if y < 1.0 { y } else { 1.0 };
    //         (255.0 * y) as u8
    //     }).collect()
    // }
}
