use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::process::Command;

use glib::clone;
use gtk::glib;
use gtk::prelude::*;
extern crate system_shutdown;
use system_shutdown::ShutdownResult;
use system_shutdown::{reboot, shutdown};

fn main() {
    let application = gtk::Application::new(Some("com.github.rew-shutdown"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}

pub fn get_sessionid() -> Result<String , Box<dyn std::error::Error>> {
    let file = File::open("/proc/self/sessionid")?;
    let mut buffered = BufReader::new(file);
    let mut sessionid = String::new();
    buffered.read_line(&mut sessionid)?;
    Ok(sessionid)
}

pub fn rew_logout() -> ShutdownResult {
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

fn dbus_send(singnal :&str, context :&str) -> ShutdownResult {
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

pub fn rew_hibernate() -> ShutdownResult {
    dbus_send("Hibernate", "boolean:true")
}

pub fn rew_suspend() -> ShutdownResult {
    dbus_send("Suspend", "boolean:true")
}


pub fn rew_lock() -> ShutdownResult {
    let sessionid = get_sessionid().unwrap();
    let mut context = String::from("string:");
    context.push_str(&sessionid);
    dbus_send("LockSession", context.as_str())
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("shutdown"));
    window.set_default_size(200, 120);

    // Here we construct the grid that is going contain our buttons.
    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    // Add the grid in the window
    window.set_child(Some(&grid));

    let button_logout = gtk::Button::with_label("logout");
    button_logout.connect_clicked(move |_| match rew_logout() {
        Ok(_) => println!("Logout, bye!"),
        Err(error) => eprintln!("Failed to logout: {}", error),
    });

    grid.attach(&button_logout, 0, 0, 1, 1);

    let button_shutdown = gtk::Button::with_label("shutdown");
    button_shutdown.connect_clicked(move |_| match shutdown() {
        Ok(_) => println!("Shutting down, bye!"),
        Err(error) => eprintln!("Failed to shut down: {}", error),
    });

    grid.attach(&button_shutdown, 1, 0, 1, 1);

    let button_reboot = gtk::Button::with_label("reboot");
    button_reboot.connect_clicked(move |_| match reboot() {
        Ok(_) => println!("reboot, bye!"),
        Err(error) => eprintln!("Failed to reboot: {}", error),
    });

    grid.attach(&button_reboot, 2, 0, 1, 1);

    let button_hibernate = gtk::Button::with_label("hibernate");
    button_hibernate.connect_clicked(move |_| match rew_hibernate() {
        Ok(_) => println!("Hibernate, bye!"),
        Err(error) => eprintln!("Failed to hibernate: {}", error),
    });
    grid.attach(&button_hibernate, 3, 0, 1, 1);

    let button_suspend = gtk::Button::with_label("suspend");
    button_suspend.connect_clicked(move |_| match rew_suspend() {
        Ok(_) => println!("Suspend, bye!"),
        Err(error) => eprintln!("Failed to Suspend: {}", error),
    });
    grid.attach(&button_suspend, 4, 0, 1, 1);

    let button_lock = gtk::Button::with_label("lock");
    button_lock.connect_clicked(move |_| match rew_lock() {
        Ok(_) => println!("Lock,bye!"),
        Err(error) => eprintln!("Failed to Lock: {}", error),
    });
    grid.attach(&button_lock, 5, 0, 1, 1);

    // Create the quit button and put it into the grid at (0, 1)
    let quit_button = gtk::Button::with_label("Quit");
    quit_button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    grid.attach(&quit_button, 0, 1, 2, 1);

    window.show();
}
