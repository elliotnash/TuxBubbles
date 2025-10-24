use std::fmt;

use gettextrs::gettext;
use libadwaita::prelude::{EntryRowExt, PreferencesRowExt};
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    actions::ActionName,
    adw,
    gtk::{
        self,
        prelude::{ActionableExt, BoxExt, ButtonExt, OrientableExt, WidgetExt},
    },
};

use crate::{
    app::{AboutAction, PreferencesAction, ShortcutsAction},
    config::APP_ID,
};

#[derive(Debug, PartialEq)]
enum OnboardingStep {
    Welcome,
    Connection,
    Sync,
}

impl fmt::Display for OnboardingStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl OnboardingStep {
    fn next(&self) -> Self {
        match self {
            Self::Welcome => Self::Connection,
            _ => Self::Sync,
        }
    }
    fn previous(&self) -> Self {
        match self {
            Self::Sync => Self::Connection,
            _ => Self::Welcome,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OnboardingPageMsg {
    Next,
    Previous,
}

pub struct OnboardingPage {
    step: OnboardingStep,
    transition: gtk::StackTransitionType,
}

#[relm4::component(pub)]
impl SimpleComponent for OnboardingPage {
    type Input = OnboardingPageMsg;
    type Output = ();
    type Init = ();

    view! {
        #[root]
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                pack_start = &gtk::Revealer {
                    #[watch]
                    set_reveal_child: model.step != OnboardingStep::Welcome,
                    set_transition_type: gtk::RevealerTransitionType::Crossfade,

                    gtk::Button {
                      set_tooltip_text: Some(&gettext("Previous Page")),
                      set_icon_name: "go-previous-symbolic",
                      connect_clicked => OnboardingPageMsg::Previous,
                    }
                },

                pack_end = &gtk::Button {
                    set_icon_name: "help-about-symbolic",
                    set_action_name: Some(&AboutAction::action_name()),
                    set_tooltip_text: Some(&gettext("About TuxBubbles"))
                }
            },

            set_content: Some(&stack)

        },
        stack = &gtk::Stack {
            add_titled: (&welcome, Some(&OnboardingStep::Welcome.to_string()), &gettext("Welcome to TuxBubbles")),
            add_titled: (&connection, Some(&OnboardingStep::Connection.to_string()), &gettext("Connect to your BlueBubbles instance")),
            add_titled: (&sync, Some(&OnboardingStep::Sync.to_string()), &gettext("Sync your message")),

            #[watch]
            set_transition_type: model.transition,

            #[watch]
            set_visible_child_name: &model.step.to_string()
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
                    connect_clicked => OnboardingPageMsg::Next,
                }
            }
        },
        connection = &adw::StatusPage {
            set_title: &gettext("Connect to your BlueBubbles instance"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: false,
                set_halign: gtk::Align::Center,
                set_spacing: 36,

                gtk::ListBox {
                    add_css_class: "boxed-list",

                    adw::EntryRow {
                        set_input_purpose: gtk::InputPurpose::Url,
                        set_title: &gettext("BlueBubbles server URL")
                    },

                    adw::PasswordEntryRow {
                        set_input_purpose: gtk::InputPurpose::Password,
                        set_title: &gettext("BlueBubbles server password")
                    }
                },

                gtk::Button {
                    set_label: &gettext("Connect"),
                    set_halign: gtk::Align::Center,
                    set_width_request: 120,
                    add_css_class: "suggested-action",
                    add_css_class: "pill",
                    connect_clicked => OnboardingPageMsg::Next,
                }
            }
        },
        sync = &adw::StatusPage {
            set_title: &gettext("Sync your messages")
        }
    }

    fn init(_: Self::Init, root: Self::Root, __: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = Self {
            step: OnboardingStep::Welcome,
            transition: gtk::StackTransitionType::SlideLeft,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        // println!("Sent a message");
        match message {
            OnboardingPageMsg::Next => {
                self.step = self.step.next();
                self.transition = gtk::StackTransitionType::SlideLeft;
            }
            OnboardingPageMsg::Previous => {
                self.step = self.step.previous();
                self.transition = gtk::StackTransitionType::SlideRight;
            }
        }
    }
}
