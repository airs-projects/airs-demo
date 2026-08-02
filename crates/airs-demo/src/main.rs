use airs::window::WindowLoop;
use airs_demo_core::AirsDemoCore;
use airs_demo_gui::MainWindow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let core = AirsDemoCore::new()?;
    let main_window = MainWindow::new(core);
    WindowLoop::new().run(main_window)?;
    Ok(())
}
