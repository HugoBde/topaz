use crate::my_image::MyImage;

#[derive(Debug, Clone)]
pub enum Message {
  ImageLoaded(MyImage),
  ImageNotFound,
}
