use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::process::Command;

pub type ShutdownResult = io::Result<()>;

fn get_sessionid() -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open("/proc/self/sessionid")?;
    let mut buffered = BufReader::new(file);
    let mut sessionid = String::new();
    buffered.read_line(&mut sessionid)?;
    Ok(sessionid)
}

fn dbus_send(singnal: &str, context: &str) -> ShutdownResult {
    let mut cmd = Command::new("dbus-send");
    cmd.arg("--system")
        .arg("--print-reply")
        .arg("--dest=org.freedesktop.login1")
        .arg("/org/freedesktop/login1")
        .arg("org.freedesktop.login1.Manager.".to_owned() + singnal)
        .arg(context);
    match cmd.output() {
        Ok(output) => {
            if output.status.success() && output.stderr.is_empty() {
                return Ok(());
            }
            Err(Error::new(
                ErrorKind::Other,
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
        Err(error) => Err(error),
    }
}

pub fn logout() -> ShutdownResult {
    let sessionid = get_sessionid().unwrap();
    let mut cmd = Command::new("loginctl");
    cmd.arg("terminate-session").arg(sessionid);
    match cmd.output() {
        Ok(output) => {
            if output.status.success() && output.stderr.is_empty() {
                return Ok(());
            }
            Err(Error::new(
                ErrorKind::Other,
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
        Err(error) => Err(error),
    }
}

pub fn hibernate() -> ShutdownResult {
    dbus_send("Hibernate", "boolean:true")
}

pub fn suspend() -> ShutdownResult {
    dbus_send("Suspend", "boolean:true")
}

pub fn lock() -> ShutdownResult {
    let sessionid = get_sessionid().unwrap();
    let mut context = String::from("string:");
    context.push_str(&sessionid);
    dbus_send("LockSession", context.as_str())
}

pub fn shutdown() -> ShutdownResult {
    dbus_send("PowerOff", "boolean:true")
}

pub fn reboot() -> ShutdownResult {
    dbus_send("Reboot", "boolean:true")
}

