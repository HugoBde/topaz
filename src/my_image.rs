use iced::widget::image::Handle;
use image::{ImageBuffer, Rgba};

#[derive(Debug, Clone)]
pub struct MyImage(pub ImageBuffer<Rgba<u8>, Vec<u8>>);

impl Into<Handle> for MyImage {
  fn into(self) -> Handle {
    let width = self.0.width();
    let height = self.0.height();

    Handle::from_rgba(width, height, self.0.into_raw())
  }
}

const DITHER_MATRIX_4x4: [[f32; 4]; 4] = [
  [0.0 / 16.0, 8.0 / 16.0, 2.0 / 16.0, 10.0 / 16.0],
  [12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0, 6.0 / 16.0],
  [3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0],
  [15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0],
];

impl MyImage {
  pub fn ordered_dither(&mut self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let i = ImageBuffer::from_raw(
      self.0.width(),
      self.0.height(),
      self
        .0
        .pixels()
        .map(|pixel| {
          [
            pixel[0] & 0xF6,
            pixel[1] & 0xF6,
            pixel[2] & 0xF6,
            pixel[3] & 0xF6,
          ]
        })
        .collect(),
    )
    .unwrap();

    return i;
  }
}
