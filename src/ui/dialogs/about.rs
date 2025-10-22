use relm4::{adw::{self, prelude::AdwDialogExt}, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::config::{APP_ID, VERSION};

pub struct AboutDialog {
    parent: Option<adw::ApplicationWindow>,
}

impl SimpleComponent for AboutDialog {
    type Init = Option<adw::ApplicationWindow>;
    type Widgets = adw::AboutDialog;
    type Input = ();
    type Output = ();
    type Root = adw::AboutDialog;

    fn init_root() -> Self::Root {
        adw::AboutDialog::builder()
            .application_name("TuxBubbles")
            .application_icon(APP_ID)
            .website("https://github.com/elliotnash/TuxBubbles")
            .issue_url("https://github.com/elliotnash/TuxBubbles/issues/new")
            .version(VERSION)
            .translator_credits("translator-credits")
            .copyright("© 2025 Elliot Nash")
            .developers(vec!["Elliot Nash"])
            .license_type(gtk::License::Gpl30)
            .build()
    }

    fn init(
        parent: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { parent };

        let widgets = root.clone();

        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.present(self.parent.as_ref());
    }
}
