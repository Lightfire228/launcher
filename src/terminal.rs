use dbus::blocking::{Connection, Proxy};
use std::{collections::HashSet, time::{Duration, Instant}};
use crate::exec;

#[derive(Debug)]
pub struct TerminalInstance {
    pub dbus_session: String,
    pub pid:          u32,
}


pub fn list_session_names() -> Vec<String> {

    let conn  = Connection::new_session().unwrap();
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    _get_session_names(&proxy)

}


pub fn new_window(path: &str) -> TerminalInstance {


    let conn   = Connection::new_session().unwrap();
    let proxy  = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
    let filter = |x: &String| x.contains("konsole");

    let names_before = _get_session_names_by(&proxy, filter);
    let names_before = _to_set(names_before);

    exec::open_terminal(path);

    let start = Instant::now();
    let dbus_session = loop {
        let names_after = _get_session_names_by(&proxy, filter);
        let unique      = _diff(&names_before, &names_after);

        assert!(unique.len() <= 1, "Multiple new DBus ids were found for spawned console session");

        if !unique.is_empty() {
            break unique.first().unwrap().to_owned();
        }

        if start.elapsed().as_secs() > 1 {
            panic!("Could not find DBus id for spawned konsole session");
        }
    };

    TerminalInstance {
        pid: _get_pid(&proxy, &dbus_session),
        dbus_session,
    }
}


fn _get_session_names(proxy: &Proxy<&Connection>) -> Vec<String> {
    _get_session_names_by(proxy, |_| true)
}

fn _get_session_names_by<T>(proxy: &Proxy<&Connection>, filter: T) -> Vec<String>
    where T: Fn(&String) -> bool
{
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ()).unwrap();

    names.into_iter().filter(filter).collect()
}

fn _to_set(names: Vec<String>) -> HashSet<String> {
    names.into_iter().collect()
}

fn _diff(before: &HashSet<String>, after: &[String]) -> Vec<String> {
    after.iter().filter(|x| !before.contains(*x)).cloned().collect()
}

fn _get_pid(proxy: &Proxy<&Connection>, name: &str) -> u32 {
    let (pid,): (u32,) = proxy.method_call("org.freedesktop.DBus", "GetConnectionUnixProcessID", (name,)).unwrap();

    pid
}
