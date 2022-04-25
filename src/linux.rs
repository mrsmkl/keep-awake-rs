pub struct Holder {
    _conn: Option<zbus::blocking::Connection>,
}

// const GNOME_INHIBIT_LOGOUT: u32 = 1;
// const GNOME_INHIBIT_SWITCH_USER: u32 = 2;
const GNOME_INHIBIT_SUSPEND_SESSION: u32 = 4;
const GNOME_INHIBIT_MARK_SESSION_IDLE: u32 = 8;

fn name_has_owner(name: &str) -> bool {
    if let Ok(conn) = zbus::blocking::Connection::session() {
        let reply = conn
            .call_method(
                Some("org.freedesktop.DBus"),
                "/",
                Some("org.freedesktop.DBus"),
                "NameHasOwner",
                &(name),
            );
        return reply.and_then(|r| r.body()).unwrap_or(false);
    }
    false
}

pub fn inhibit_display(name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    if name_has_owner("org.gnome.SessionManager") {
        if let Ok(conn) = zbus::blocking::Connection::session() {
            let flags: u32 = GNOME_INHIBIT_SUSPEND_SESSION | GNOME_INHIBIT_MARK_SESSION_IDLE;
            let _reply = conn
                .call_method(
                    Some("org.gnome.SessionManager"),
                    "/org/gnome/SessionManager",
                    Some("org.gnome.SessionManager"),
                    "Inhibit",
                    &(name, 0, reason, flags),
                );
            return Ok(Holder { _conn: Some(conn) })
        }
    }

    match zbus::blocking::Connection::session() {
        Ok(conn) => {
            let _reply = conn
                .call_method(
                    Some("org.freedesktop.ScreenSaver"),
                    "/org/freedesktop/ScreenSaver",
                    Some("org.freedesktop.ScreenSaver"),
                    "Inhibit",
                    &(name, reason),
                );
            Ok(Holder { _conn: Some(conn) })
        }
        _ => Ok(Holder { _conn: None }),
    }
}

pub fn inhibit_system(name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    if name_has_owner("org.gnome.SessionManager") {
        if let Ok(conn) = zbus::blocking::Connection::session() {
            let flags: u32 = GNOME_INHIBIT_SUSPEND_SESSION;
            let _reply = conn
                .call_method(
                    Some("org.gnome.SessionManager"),
                    "/org/gnome/SessionManager",
                    Some("org.gnome.SessionManager"),
                    "Inhibit",
                    &(name, 0, reason, flags),
                );
            return Ok(Holder { _conn: Some(conn) })
        }
    }

    match zbus::blocking::Connection::session() {
        Ok(conn) => {
            let _reply = conn
                .call_method(
                    Some("org.freedesktop.PowerManagement"),
                    "/org/freedesktop/PowerManagement/Inhibit",
                    Some("org.freedesktop.PowerManagement.Inhibit"),
                    "Inhibit",
                    &(name, reason),
                );
            Ok(Holder { _conn: Some(conn) })
        }
        _ => Ok(Holder { _conn: None }),
    }
}
