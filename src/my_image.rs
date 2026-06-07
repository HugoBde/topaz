#![allow(unused_variables)]
use iced::widget::image::Handle;
use image::{Rgba, RgbaImage};

#[derive(Debug, Clone)]
pub struct MyImage {
  pub inner:  RgbaImage,
  pub dither: u8,
}

impl Into<Handle> for MyImage {
  fn into(self) -> Handle {
    let width = self.inner.width();
    let height = self.inner.height();

    Handle::from_rgba(width, height, self.inner.into_raw())
  }
}

const DITHER_MATRIX_4X4: [[f32; 4]; 4] = [
  [0.0 / 16.0, 8.0 / 16.0, 2.0 / 16.0, 10.0 / 16.0],
  [12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0, 6.0 / 16.0],
  [3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0],
  [15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0],
];

const DITHER_MATRIX_2X2: [[f32; 2]; 2] = [[0.0 / 16.0, 2.0 / 16.0], [3.0 / 16.0, 1.0 / 16.0]];

impl MyImage {
  pub fn ordered_dither(&self) -> MyImage {
    // let new_dither = self.dither + 1;
    let mask: u8 = 0xFF ^ ((1 << 5) - 1);
    let width = self.inner.width();
    let height = self.inner.height();

    let i = RgbaImage::from_vec(
      width,
      height,
      self
        .inner
        .enumerate_pixels()
        .map(|(x, y, Rgba(pixel))| {
          let x = x as usize % 2;
          let y = y as usize % 2;
          [
            // (pixel[0] & mask),
            // (pixel[1] & mask),
            // (pixel[2] & mask),
            // (pixel[3] & mask),
            ((pixel[0] as f32 + DITHER_MATRIX_2X2[x][y]) as u8 & mask),
            ((pixel[1] as f32 + DITHER_MATRIX_2X2[x][y]) as u8 & mask),
            ((pixel[2] as f32 + DITHER_MATRIX_2X2[x][y]) as u8 & mask),
            ((pixel[3] as f32 + DITHER_MATRIX_2X2[x][y]) as u8 & mask),
            // ((pixel[0] & mask) as f32 + DITHER_MATRIX_4X4[x][y]) as u8,
            // ((pixel[1] & mask) as f32 + DITHER_MATRIX_4X4[x][y]) as u8,
            // ((pixel[2] & mask) as f32 + DITHER_MATRIX_4X4[x][y]) as u8,
            // ((pixel[3] & mask) as f32 + DITHER_MATRIX_4X4[x][y]) as u8,
          ]
        })
        .flatten()
        .collect(),
    )
    .unwrap();
    return MyImage {
      inner:  i,
      dither: 0,
    };
  }
}
