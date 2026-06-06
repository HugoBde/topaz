use iced::widget::image::Handle;
use image::DynamicImage;

#[derive(Debug, Clone)]
pub struct MyImage(pub DynamicImage);

impl Into<Handle> for MyImage {
  fn into(self) -> Handle {
    let width = self.0.width();
    let height = self.0.height();

    Handle::from_rgba(width, height, self.0.into_rgba8().into_raw())
  }
}
