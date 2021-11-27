use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};
use adw::subclass::prelude::AdwApplicationWindowImpl;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/jannuary/Paroles/window.ui")]
    pub struct ParolesWindow {
        // Template widgets
        #[template_child]
        pub window_header: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub window_title: TemplateChild<adw::WindowTitle>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ParolesWindow {
        const NAME: &'static str = "ParolesWindow";
        type Type = super::ParolesWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ParolesWindow {}
    impl WidgetImpl for ParolesWindow {}
    impl WindowImpl for ParolesWindow {}
    impl ApplicationWindowImpl for ParolesWindow {}
    impl AdwApplicationWindowImpl for ParolesWindow {}
}

glib::wrapper! {
    pub struct ParolesWindow(ObjectSubclass<imp::ParolesWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ParolesWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create ParolesWindow")
    }
}
