






mod config;
mod exec;
mod menu;
mod terminal;

fn main() {
    println!("Hello, world!");

    let config = config::get_config();
    let proj   = menu  ::display_menu(&config);

    exec::launch_project(proj);
}
