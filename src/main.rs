use app::App;

mod app;
mod models;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut app = App::new();

  println!("{:?}", app);
  Ok(())
}
