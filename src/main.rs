mod config;

use gtk::prelude::*;
use relm4::prelude::*;
use gtk::{gdk, glib, CssProvider};
use gtk4_layer_shell::{Edge, Layer, LayerShell, KeyboardMode};
use config::{Config, execute_action};

const APP_ID: &str = "com.github.rew-shutdown";

#[derive(Debug)]
enum AppInput {
    ExecuteAction(String),
    Quit,
}

struct App {
    config: Config,
}

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
        }
    }

    // Initialize the component.
    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        load_css();
        
        // Load configuration
        let config = Config::load_from_kdl().expect("Failed to load configuration");
        
        // Debug: print loaded keybinds
        println!("Loaded keybinds:");
        for button_config in &config.buttons {
            println!("  '{}' -> {} ({})", 
                     button_config.keybind, 
                     button_config.text, 
                     button_config.action);
        }
        
        let model = App { config };
        
        // Create dynamic UI based on configuration
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 20);
        main_box.set_halign(gtk::Align::Center);
        main_box.set_valign(gtk::Align::Center);
        
        // Create rows of buttons (3 buttons per row)
        let mut current_row: Option<gtk::Box> = None;
        let mut button_count = 0;
        
        for button_config in &model.config.buttons {
            // Create new row if needed
            if button_count % 3 == 0 {
                let row = gtk::Box::new(gtk::Orientation::Horizontal, 20);
                row.set_halign(gtk::Align::Center);
                main_box.append(&row);
                current_row = Some(row);
            }
            
            if let Some(ref row) = current_row {
                let button = gtk::Button::with_label(&button_config.text);
                button.add_css_class("button");
                button.set_size_request(100, 100);
                button.set_can_focus(true);
                
                // Clone action for the closure
                let action = button_config.action.clone();
                let sender_clone = sender.clone();
                button.connect_clicked(move |_| {
                    sender_clone.input(AppInput::ExecuteAction(action.clone()));
                });
                
                row.append(&button);
                button_count += 1;
            }
        }
        
        root.set_child(Some(&main_box));
        
        // Setup key event controller
        let key_controller = gtk::EventControllerKey::new();
        let sender_clone = sender.clone();
        let config_clone = model.config.buttons.clone();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                sender_clone.input(AppInput::Quit);
            } else {
                // Check for keybind matches using keyval
                let key_char = key.to_unicode();
                if let Some(ch) = key_char {
                    let key_str = ch.to_lowercase().to_string();
                    for button_config in &config_clone {
                        if button_config.keybind.to_lowercase() == key_str {
                            println!("Triggered keybind '{}' for action: {}", button_config.keybind, button_config.action);
                            sender_clone.input(AppInput::ExecuteAction(button_config.action.clone()));
                            break;
                        }
                    }
                } else {
                    // 对于非字符键，使用键名匹配
                    let key_name = key.name().unwrap_or_default().to_lowercase();
                    for button_config in &config_clone {
                        if button_config.keybind.to_lowercase() == key_name {
                            println!("Triggered keybind '{}' for action: {}", button_config.keybind, button_config.action);
                            sender_clone.input(AppInput::ExecuteAction(button_config.action.clone()));
                            break;
                        }
                    }
                }
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

        // Setup hover focus for all buttons
        setup_button_hover_focus(&root);

        // Create empty widgets for compatibility
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::ExecuteAction(action) => {
                match execute_action(&action) {
                    Ok(_) => {
                        println!("Executed: {}", action);
                        // 执行命令成功后退出应用
                        std::process::exit(0);
                    }
                    Err(error) => {
                        eprintln!("Failed to execute '{}': {}", action, error);
                        // 执行失败也退出应用，避免留在后台
                        std::process::exit(1);
                    }
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

fn setup_button_hover_focus(window: &gtk::ApplicationWindow) {
    // 递归查找所有按钮并添加 hover 事件
    fn find_and_setup_buttons(widget: &gtk::Widget) {
        if let Some(button) = widget.downcast_ref::<gtk::Button>() {
            let motion_controller = gtk::EventControllerMotion::new();
            let button_clone = button.clone();
            motion_controller.connect_enter(move |_, _, _| {
                button_clone.grab_focus();
            });
            button.add_controller(motion_controller);
        }
        
        // 如果是容器，递归处理其子组件  
        if let Some(container) = widget.downcast_ref::<gtk::Box>() {
            let mut child = container.first_child();
            while let Some(widget) = child {
                find_and_setup_buttons(&widget);
                child = widget.next_sibling();
            }
        }
    }
    
    find_and_setup_buttons(window.upcast_ref::<gtk::Widget>());
}
