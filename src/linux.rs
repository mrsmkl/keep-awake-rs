pub struct Holder {
    _conn: zbus::Connection,
}

pub fn inhibit(name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    let conn = zbus::Connection::new_session().unwrap();
    let _reply = conn
        .call_method(
            Some("org.freedesktop.ScreenSaver"),
            "/org/freedesktop/ScreenSaver",
            Some("org.freedesktop.ScreenSaver"),
            "Inhibit",
            &(name, reason),
        )
        .unwrap();
    Ok(Holder { _conn: conn })
}
