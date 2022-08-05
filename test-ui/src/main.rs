mod main_window;

use main_window::MainWindow;

use gtk::{
    gio::resources_register_include,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::WidgetExt,
};
use adw::Application;

fn build_ui(application: &Application) {
    let window = MainWindow::new(application);
    window.show();
}

pub fn main() {
    resources_register_include!("test-ui.gresource").expect("Failed to register resources.");

    let application = Application::builder()
        .application_id("com.asrifox.adwaita-test-ui")
        .build();

    application.connect_activate(build_ui);
    application.run();
}