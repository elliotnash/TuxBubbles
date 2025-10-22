use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup}, adw, gtk, main_application, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmIterChildrenExt, SimpleComponent
};

use gtk::prelude::{
    ApplicationExt, GtkWindowExt, OrientableExt, SettingsExt, WidgetExt,
};
use gtk::{gio, glib};

use crate::{config::{APP_ID, PROFILE}, ui::pages::onboarding::OnboardingPage};
use crate::ui::dialogs::about::AboutDialog;
use crate::ui::dialogs::shortcuts::ShortcutsDialog;

pub(super) struct App {
    about_dialog: Controller<AboutDialog>,
    shortcuts_dialog: Controller<ShortcutsDialog>,
    // onboarding_page: Controller<OnboardingPage>
}

#[derive(Debug)]
pub(super) enum AppMsg {
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(pub(super) PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(pub(super) AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About TuxBubbles" => AboutAction,
            }
        }
    }

    view! {
        #[root]
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },


            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

            // stack {}
            #[name = "toast_overlay"]
            adw::ToastOverlay {
                set_child: Some(&main_stack)
            }
        },
        main_stack = &gtk::Stack {
            add_titled: (onboarding_page.widget(), Some("onboarding"), "Onboarding"),
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let about_dialog = AboutDialog::builder()
            .launch(Some(root.clone()))
            .detach();

        let shortcuts_dialog = ShortcutsDialog::builder()
            .launch(Some(root.clone()))
            .detach();

        let onboarding_page = OnboardingPage::builder().launch(());

        let model = Self {
            // onboarding_page,
            about_dialog,
            shortcuts_dialog,
        };

        let widgets = view_output!();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            let sender = model.shortcuts_dialog.sender().clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                sender.send(()).unwrap();
            })
        };
        main_application().set_accelerators_for_action::<ShortcutsAction>(&["<Control>question"]);

        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(()).unwrap();
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => main_application().quit(),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
