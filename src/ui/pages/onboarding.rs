use gettextrs::gettext;
use relm4::{actions::{ActionName}, adw, gtk::{self, prelude::{ActionableExt, BoxExt, ButtonExt, OrientableExt, WidgetExt}}, ComponentParts, ComponentSender, SimpleComponent};

use crate::{app::{AboutAction, PreferencesAction, ShortcutsAction}, config::APP_ID};

pub struct OnboardingPage {}

#[relm4::component(pub)]
impl SimpleComponent for OnboardingPage {
    type Input = ();
    type Output = ();
    type Init = ();

    view! {
        #[root]
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                pack_end = &gtk::Button {
                    set_icon_name: "help-about-symbolic",
                    set_action_name: Some(&AboutAction::action_name()),
                    set_tooltip_text: Some(&gettext("About TuxBubbles"))
                }
            },

            set_content: Some(&stack)

        },
        stack = &gtk::Stack {
            add_titled: (&welcome, Some("welcome"), &gettext("Welcome to TuxBubbles")),
        },
        welcome = &adw::StatusPage {
            set_icon_name: Some(APP_ID),
            set_title: &gettext("Welcome to TuxBubbles"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: false,
                set_halign: gtk::Align::Center,
                set_spacing: 12,

                gtk::Button {
                    set_label: &gettext("Continue"),
                    set_halign: gtk::Align::Center,
                    add_css_class: "suggested-action",
                    add_css_class: "pill",
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        __: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = OnboardingPage {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        // match message {

        // }
    }
}
