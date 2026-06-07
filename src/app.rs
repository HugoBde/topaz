use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use iced::Length;
use iced::Task;
use iced::widget;
use iced::widget::button;
use iced::window;
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

const TARGET_HEIGHT: u32 = 700;

impl App {
  pub fn new() -> (App, Task<Message>) {
    (App::Loading, Task::perform(App::zob(), App::zab))
  }

  pub async fn zob() -> Result<MyImage> {
    let args = Args::parse();
    let image = ImageReader::open(args.image)?.decode()?;
    let rsz_factor = image.height() / TARGET_HEIGHT;
    let image = MyImage {
      inner:  image
        .resize(
          image.width() / rsz_factor,
          image.height() / rsz_factor,
          image::imageops::FilterType::Nearest,
        )
        .grayscale()
        .to_rgba8(),
      dither: 0,
    };

    Ok(image)
  }

  pub fn zab(boot_res: Result<MyImage>) -> Message {
    match boot_res {
      Ok(image) => Message::ImageLoaded(image),
      Err(e) => {
        eprintln!("{e}");
        Message::ImageNotFound
      }
    }
  }

  pub fn view(&self) -> widget::Column<'_, Message> {
    match self {
      Self::Loading => widget::column![widget::text("loading...")],
      Self::Failed => widget::column![widget::text("failed")],
      Self::Loaded(image) => widget::column![
        widget::image(image.clone()),
        button("Dither")
          .height(Length::Fill)
          .width(Length::Fill)
          .on_press_with(move || { Message::ImageLoaded(image.ordered_dither()) })
      ],
    }
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::ImageLoaded(image) => {
        let width = image.inner.width() as f32;
        let height = image.inner.height() as f32;
        *self = Self::Loaded(image);
        window::latest().and_then(move |id| {
          window::resize(
            id,
            iced::Size {
              width,
              height: height + 30.,
            },
          )
        })
      }
      Message::ImageNotFound => {
        *self = Self::Failed;
        Task::none()
      }
    }
  }
}
