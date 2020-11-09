pub struct Holder {
    _conn: Option<zbus::Connection>,
}

pub fn inhibit(name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    match zbus::Connection::new_session() {
        Ok(conn) => {
            let _reply = conn
            .call_method(
            Some("org.freedesktop.ScreenSaver"),
            "/org/freedesktop/ScreenSaver",
            Some("org.freedesktop.ScreenSaver"),
            "Inhibit",
            &(name, reason),
        )
        .unwrap();
        Ok(Holder { _conn: Some(conn) })
        }
        _ => Ok(Holder { _conn: None }),
    }
}
