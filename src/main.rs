mod cmds;
mod app;

use crate::app::MyApp;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "DD Gui",
        eframe::NativeOptions::default(),
        Box::new(|ctx| Ok(Box::new(MyApp::new(ctx)))),
    )
}

