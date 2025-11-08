
mod config;
mod exec;
mod menu;
mod terminal;
mod dbus_codegen;

fn main() {
    let config = config::get_config();
    let proj   = menu  ::display_menu(&config);

    exec::launch_project(proj);
}
