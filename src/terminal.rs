use dbus::blocking::Connection;
use std::{collections::HashSet, ops::Deref, thread::sleep, time::Duration};
use crate::exec;

#[derive(Debug)]
pub struct TerminalInstance {
    pub dbus_session: String,
    pub pid:          u32,
}


pub fn list_names() -> Result<(), Box<dyn std::error::Error>> {

    let conn  = Connection::new_session()?;
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    for name in names { println!("{}", name); }

    Ok(())
}


pub fn new_window() -> TerminalInstance {


    let conn  = Connection::new_session().unwrap();
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ()).unwrap();

    let     names_list = names.into_iter().filter(|x| x.contains("konsole")).collect::<Vec<String>>();
    let mut names_set  = HashSet::new();

    names_list.into_iter().for_each(|x| { names_set.insert(x); });

    exec::open_terminal("");
    sleep(Duration::from_secs(1)); // TODO:

    let (new_names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ()).unwrap();

    let new_names = new_names.into_iter().filter(|x| x.contains("konsole"));

    let unique: Vec<String> = new_names.into_iter().filter(|x| !names_set.contains(x)).collect();

    assert!(unique.len() <= 1, "Multiple new DBus ids were found for spawned console session");

    let dbus_session = unique.first().expect("Could not find DBus id for spawned konsole session").to_owned();

    let (pid,): (u32,) = proxy.method_call("org.freedesktop.DBus", "GetConnectionUnixProcessID", (&dbus_session,)).unwrap();

    TerminalInstance {
        dbus_session,
        pid,
    }


}
