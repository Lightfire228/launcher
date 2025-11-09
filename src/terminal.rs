use dbus::blocking::{Connection, Proxy};
use std::{collections::HashSet, process::Command, time::{Duration, Instant}};

pub fn new_window(path: &str) -> TerminalInstance {

    let conn   = Connection::new_session().unwrap();
    let proxy  = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
    let filter = |x: &String| x.contains("konsole");

    let names_before = _get_session_names_by(&proxy, filter);
    let names_before = _to_set(names_before);

    _launch_terminal(path);

    let start = Instant::now();
    let dbus_session = loop {
        assert!(start.elapsed().as_millis() < 1000, "Could not find DBus id for spawned konsole session");

        let names_after = _get_session_names_by(&proxy, filter);
        let unique      = _diff(&names_before, &names_after);

        assert!(unique.len() <= 1, "Multiple new DBus ids were found for spawned console session");

        if !unique.is_empty() {
            break unique.first().unwrap().to_owned();
        }
    };

    TerminalInstance {
        pid:        _get_pid(&proxy, &dbus_session),
        connection: conn,
        dbus_session,
    }
}


#[allow(dead_code)]
pub struct TerminalInstance {
    pub dbus_session: String,
    pub pid:          u32,

    connection: Connection,
}

impl TerminalInstance {
    pub fn new_tab(&self, dir: &str) {
        use crate::dbus_codegen::konsole_windows_1::OrgKdeKonsoleWindow;

        let proxy = self.get_proxy("/Windows/1");

        proxy.new_session__("", dir).expect("Unable to spawn new tab");
    }

    fn get_proxy<'a>(&'a self, path: &'a str) -> Proxy<'a, &'a Connection> {
        self.connection.with_proxy(&self.dbus_session, path, Duration::from_millis(5000))
    }
}


fn _get_session_names(proxy: &Proxy<&Connection>) -> Vec<String> {
    _get_session_names_by(proxy, |_| true)
}

fn _get_session_names_by<T>(proxy: &Proxy<&Connection>, filter: T) -> Vec<String>
    where T: Fn(&String) -> bool
{
    use crate::dbus_codegen::org_freedesktop_dbus::OrgFreedesktopDBus;

    proxy
        .list_names()
        .expect("Unable to list dbus session names")
        .into_iter()
        .filter(filter)
        .collect()
}

fn _to_set(names: Vec<String>) -> HashSet<String> {
    names.into_iter().collect()
}

fn _diff(before: &HashSet<String>, after: &[String]) -> Vec<String> {
    after.iter().filter(|x| !before.contains(*x)).cloned().collect()
}

fn _get_pid(proxy: &Proxy<&Connection>, name: &str) -> u32 {

    use crate::dbus_codegen::org_freedesktop_dbus::OrgFreedesktopDBus;

    proxy
        .get_connection_unix_process_id(name)
        .unwrap_or_else(|_| panic!("Unable to get PID of {name}"))

}


fn _launch_terminal(path: &str) {
    Command::new("systemd-run")
        .args(["--user", "konsole", "--workdir", path])
        .output()
        .expect("failed to open konsole")
    ;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_konsole() {
        new_window("/tmp");
    }

}
