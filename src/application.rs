use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use adw::subclass::prelude::AdwApplicationImpl;
use adw::ColorScheme;

use crate::config::VERSION;
use crate::ParolesWindow;

mod imp {
    use super::*;


    #[derive(Debug, Default)]
    pub struct ParolesApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for ParolesApplication {
        const NAME: &'static str = "ParolesApplication";
        type Type = super::ParolesApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for ParolesApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for ParolesApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self, application: &Self::Type) {
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = ParolesWindow::new(application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();

            application.setup_theme();
        }
    }

    impl GtkApplicationImpl for ParolesApplication {}
    impl AdwApplicationImpl for ParolesApplication {}
}

glib::wrapper! {
    pub struct ParolesApplication(ObjectSubclass<imp::ParolesApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ParolesApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new(&[("application-id", &application_id), ("flags", flags)])
            .expect("Failed to create ParolesApplication")
    }

    fn setup_gactions(&self) {
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.quit();
        }));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about();
        }));
        self.add_action(&about_action);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let dialog = gtk::AboutDialogBuilder::new()
            .transient_for(&window)
            .modal(true)
            .program_name("paroles")
            .version(VERSION)
            .authors(vec!["noëlle".into()])
            .build();

        dialog.present();
    }

    fn setup_theme(&self) {
        let manager = adw::StyleManager::default().unwrap();
        manager.set_color_scheme(ColorScheme::PreferDark);
    }
}
