use topaz::app::App;

fn main() -> iced::Result {
  iced::application(App::new, App::update, App::view)
    .window_size((600, 600))
    .resizable(false)
    .run()
}
