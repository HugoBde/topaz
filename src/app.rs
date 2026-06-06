use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use iced::Task;
use iced::widget;
use image::ImageReader;

use crate::message::Message;
use crate::my_image::MyImage;

#[derive(Debug, Parser)]
struct Args {
  #[arg(short, long)]
  image: PathBuf,
}

pub enum App {
  Loading,
  Loaded(MyImage),
  Failed,
}

impl App {
  pub fn new() -> (App, Task<Message>) {
    (App::Loading, Task::perform(App::zob(), App::zab))
  }

  pub async fn zob() -> Result<MyImage> {
    let args = Args::parse();
    let image = MyImage(ImageReader::open(args.image)?.decode()?);

    Ok(image)
  }

  pub fn zab(boot_res: Result<MyImage>) -> Message {
    match boot_res {
      Ok(image) => Message::ImageLoaded(image),
      Err(_) => Message::ImageNotFound,
    }
  }

  pub fn view(&self) -> widget::Column<'_, Message> {
    match self {
      Self::Loading => widget::column![widget::text("loading...")],
      Self::Failed => widget::column![widget::text("failed")],
      Self::Loaded(image) => widget::column![widget::image(image.clone())],
    }
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::ImageLoaded(image) => *self = Self::Loaded(image),
      Message::ImageNotFound => *self = Self::Failed,
    };
    Task::none()
  }
}
