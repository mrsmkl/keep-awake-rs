use dbus::blocking::BlockingSender;
use dbus::blocking::Connection;
use dbus::Message;
use std::time::Duration;

pub struct Holder {
    _conn: Connection,
}

pub fn inhibit(name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;

    let m = Message::new_method_call(
        "org.freedesktop.ScreenSaver",
        "/org/freedesktop/ScreenSaver",
        "org.freedesktop.ScreenSaver",
        "Inhibit",
    )?
    .append2(name, reason);
    conn.send_with_reply_and_block(m, Duration::from_millis(2000))?;
    Ok(Holder { _conn: conn })
}
