use gtk::prelude::*;
use relm4::prelude::*;
use gtk::{gdk, glib, CssProvider};
use gtk4_layer_shell::{Edge, Layer, LayerShell, KeyboardMode};
use rwlogout::{hibernate, lock, logout, reboot, shutdown, suspend};

const APP_ID: &str = "com.github.rew-shutdown";

#[derive(Debug)]
enum AppInput {
    Logout,
    Shutdown,
    Reboot,
    Hibernate,
    Suspend,
    Lock,
    Quit,
}

struct App;

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    
    view! {
        #[root]
        main_window = gtk::ApplicationWindow {
            // Initialize layer shell first
            init_layer_shell: (),
            set_layer: Layer::Overlay,
            set_namespace: "logout_dialog",
            set_keyboard_mode: KeyboardMode::OnDemand,
            set_anchor: (Edge::Top, true),
            set_anchor: (Edge::Left, true),
            set_anchor: (Edge::Bottom, true),
            set_anchor: (Edge::Right, true),
            set_exclusive_zone: -1,
            
            set_title: Some("shutdown"),
            set_default_size: (600, 400),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,
                set_spacing: 20,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 20,

                    gtk::Button {
                        set_label: "logout",
                        add_css_class: "button",
                        set_size_request: (100, 100),
                        connect_clicked => AppInput::Logout,
                    },

                    gtk::Button {
                        set_label: "shutdown", 
                        add_css_class: "button",
                        set_size_request: (100, 100),
                        connect_clicked => AppInput::Shutdown,
                    },

                    gtk::Button {
                        set_label: "reboot",
                        add_css_class: "button",
                        set_size_request: (100, 100),
                        connect_clicked => AppInput::Reboot,
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 20,

                    gtk::Button {
                        set_label: "hibernate",
                        add_css_class: "button",
                        set_size_request: (100, 100),
                        connect_clicked => AppInput::Hibernate,
                    },

                    gtk::Button {
                        set_label: "suspend",
                        add_css_class: "button",
                        set_size_request: (100, 100),
                        connect_clicked => AppInput::Suspend,
                    },

                    gtk::Button {
                        set_label: "lock",
                        add_css_class: "button",
                        set_size_request: (100, 100),
                        connect_clicked => AppInput::Lock,
                    },
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        load_css();
        
        let model = App;
        
        // Setup key event controller
        let key_controller = gtk::EventControllerKey::new();
        let sender_clone = sender.clone();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                sender_clone.input(AppInput::Quit);
            }
            glib::Propagation::Proceed
        });
        root.add_controller(key_controller);

        // Setup gesture click controller for background clicks
        let gesture_click = gtk::GestureClick::new();
        let sender_clone = sender.clone();
        gesture_click.connect_released(move |_, _, _, _| {
            sender_clone.input(AppInput::Quit);
        });
        root.add_controller(gesture_click);

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Logout => {
                match logout() {
                    Ok(_) => println!("Logout, bye!"),
                    Err(error) => eprintln!("Failed to logout: {}", error),
                }
            }
            AppInput::Shutdown => {
                match shutdown() {
                    Ok(_) => println!("Shutting down, bye!"),
                    Err(error) => eprintln!("Failed to shut down: {}", error),
                }
            }
            AppInput::Reboot => {
                match reboot() {
                    Ok(_) => println!("reboot, bye!"),
                    Err(error) => eprintln!("Failed to reboot: {}", error),
                }
            }
            AppInput::Hibernate => {
                match hibernate() {
                    Ok(_) => println!("Hibernate, bye!"),
                    Err(error) => eprintln!("Failed to hibernate: {}", error),
                }
            }
            AppInput::Suspend => {
                match suspend() {
                    Ok(_) => println!("Suspend, bye!"),
                    Err(error) => eprintln!("Failed to Suspend: {}", error),
                }
            }
            AppInput::Lock => {
                match lock() {
                    Ok(_) => println!("Lock,bye!"),
                    Err(error) => eprintln!("Failed to Lock: {}", error),
                }
            }
            AppInput::Quit => {
                std::process::exit(0);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<App>(());
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(&String::from_utf8_lossy(include_bytes!("style.css")));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
