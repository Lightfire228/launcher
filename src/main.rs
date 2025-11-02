






mod config;
mod exec;
mod menu;

fn main() {
    println!("Hello, world!");

    let config = config::get_config();

    let test = config.expand_dir("test_2", None).unwrap();

    println!(">>>> test dir: {}", test.to_str().unwrap());

    // print!("{}", config.dirs.get("work").unwrap())

    // exec::launch_project(&config.projects[0]);


}
