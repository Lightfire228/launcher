






mod config;
mod exec;
mod menu;
mod terminal;

fn main() {
    println!("Hello, world!");

    let config = config::get_config();

    let test = config.expand_dir("timekeeper", None).unwrap();

    println!(">>>> test dir: {}", test.to_str().unwrap());

    // let new_terminal = terminal::new_window("");

    // dbg!("{}", &new_terminal);

    // println!(">>>> new konsole id: {}", *terminal::new_window())
    // println!(">>>> new konsole id: {}", *terminal::new_window())



    // for name in terminal::list_session_names() {
    //     println!("{name}")
    // }


    // print!("{}", config.dirs.get("work").unwrap())

    exec::launch_project(&config, &config.projects[0]);


}
