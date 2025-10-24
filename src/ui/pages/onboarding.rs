use std::fmt;

use gettextrs::gettext;
use libadwaita::prelude::{EntryRowExt, PreferencesRowExt};
use relm4::{
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
    actions::ActionName,
    adw,
    gtk::{
        self, Adjustment,
        prelude::{
            ActionableExt, BoxExt, ButtonExt, ListBoxRowExt, OrientableExt, RangeExt, ScaleExt,
            WidgetExt,
        },
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
    NextPage,
    PreviousPage,
    Connect,
    Sync,
    ToggleSyncAll,
}

pub struct OnboardingPage {
    step: OnboardingStep,
    transition: gtk::StackTransitionType,
    sync_all: bool,
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
                      connect_clicked => OnboardingPageMsg::PreviousPage,
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
                    connect_clicked => OnboardingPageMsg::NextPage,
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
                    connect_clicked => OnboardingPageMsg::NextPage,
                }
            }
        },
        sync = &adw::StatusPage {
            set_title: &gettext("Sync your messages"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: false,
                set_halign: gtk::Align::Center,
                set_spacing: 12,
                set_width_request: 200,

                gtk::Label {
                    set_label: &gettext("How many messages do you want to sync?")
                },

                gtk::ListBox {
                    set_width_request: 400,
                    add_css_class: "boxed-list",

                    gtk::ListBoxRow {
                        set_selectable: false,
                        set_focusable: false,

                        gtk::Scale {
                            set_margin_vertical: 4,
                            set_margin_horizontal: 16,
                            set_orientation: gtk::Orientation::Horizontal,
                            set_hexpand: true,
                            #[watch]
                            set_sensitive: !model.sync_all,
                            set_adjustment: &history_scale_adjustment,

                            add_mark: (0.0, gtk::PositionType::Bottom, Some(&gettext("none"))),
                            add_mark: (25.0, gtk::PositionType::Bottom, Some(&gettext("3 mo"))),
                            add_mark: (50.0, gtk::PositionType::Bottom, Some(&gettext("6 mo"))),
                            add_mark: (75.0, gtk::PositionType::Bottom, Some(&gettext("9 mo"))),
                            add_mark: (100.0, gtk::PositionType::Bottom, Some(&gettext("1 yr")))
                        }
                    },

                    adw::SwitchRow {
                        set_title: &gettext("Sync all messages"),
                        #[watch]
                        #[block_signal(toggle_handler)]
                        set_active: model.sync_all,
                        connect_active_notify => OnboardingPageMsg::ToggleSyncAll @toggle_handler,
                    }
                },

                gtk::Box {
                    set_height_request: 12
                },

                gtk::Button {
                    set_label: &gettext("Sync"),
                    set_halign: gtk::Align::Center,
                    set_width_request: 120,
                    add_css_class: "suggested-action",
                    add_css_class: "pill",
                    connect_clicked => OnboardingPageMsg::NextPage,
                }
            }
        },
    }

    fn init(_: Self::Init, root: Self::Root, __: ComponentSender<Self>) -> ComponentParts<Self> {
        let history_scale_adjustment = gtk::Adjustment::builder()
            .upper(100.0)
            .lower(0.0)
            .step_increment(0.1)
            .build();

        let model = Self {
            step: OnboardingStep::Welcome,
            transition: gtk::StackTransitionType::SlideLeft,
            sync_all: false,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        // println!("Sent a message");
        match message {
            OnboardingPageMsg::NextPage => {
                self.step = self.step.next();
                self.transition = gtk::StackTransitionType::SlideLeft;
            }
            OnboardingPageMsg::PreviousPage => {
                self.step = self.step.previous();
                self.transition = gtk::StackTransitionType::SlideRight;
            }
            OnboardingPageMsg::ToggleSyncAll => self.sync_all = !self.sync_all,
            _ => (),
        }
    }
}
