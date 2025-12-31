use std::{fmt, sync::LazyLock};

use fancy_regex::Regex;
use gettextrs::gettext;
use libadwaita::prelude::{EntryRowExt, PreferencesRowExt};
use relm4::{
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
    actions::ActionName,
    adw,
    gtk::{
        self,
        prelude::{
            ActionableExt, BoxExt, ButtonExt, EditableExt, ListBoxRowExt, OrientableExt, RangeExt,
            ScaleExt, WidgetExt,
        },
    },
};

use crate::{
    app::{APP_BROKER, AboutAction, AppMsg},
    config::APP_ID,
};

static ALLOWED_URL_CHARS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9.\-:\/]*$").unwrap());
static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(https?:\/\/)(([a-zA-Z0-9](?:(?:[a-zA-Z0-9-]*|(?<!-)\.(?![-.]))*[a-zA-Z0-9]+)?))(:(\d+))?$").unwrap()
});

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
    ToggleSyncAll,
    UrlChanged(String),
    UrlEntered,
    PasswordChanged(String),
    Connect,
}

pub struct OnboardingPage {
    step: OnboardingStep,
    transition: gtk::StackTransitionType,
    url: String,
    url_error: Option<String>,
    password: String,
    password_error: Option<String>,
    connecting: bool,
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
                    set_css_classes: &["suggested-action", "pill"],
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

                    #[name = "url_entry"]
                    adw::EntryRow {
                        set_input_purpose: gtk::InputPurpose::Url,
                        set_title: &gettext("BlueBubbles server URL"),
                        add_controller: url_focus_controller,
                        #[watch]
                        set_tooltip_text: model.url_error.as_deref(),
                        #[watch]
                        set_css_classes: if model.url_error.is_some() { &["error"] } else { &[] },
                        connect_changed[sender] => move |entry| {
                            let text = entry.text().to_string();
                            sender.input(OnboardingPageMsg::UrlChanged(text))
                        },
                        connect_entry_activated: {
                            let password_entry = password_entry.clone();
                            move |_| {
                                password_entry.grab_focus();
                            }
                        },
                    },

                    #[name = "password_entry"]
                    adw::PasswordEntryRow {
                        set_input_purpose: gtk::InputPurpose::Password,
                        set_title: &gettext("BlueBubbles server password"),
                        #[watch]
                        set_tooltip_text: model.password_error.as_deref(),
                        #[watch]
                        set_css_classes: if model.password_error.is_some() { &["error"] } else { &[] },
                        connect_changed[sender] => move |entry| {
                            let text = entry.text().to_string();
                            sender.input(OnboardingPageMsg::PasswordChanged(text))
                        },
                        connect_entry_activated => OnboardingPageMsg::Connect,
                    }
                },

                gtk::Button {
                    set_halign: gtk::Align::Center,
                    set_width_request: 120,
                    set_css_classes: &["suggested-action", "pill"],
                    connect_clicked => OnboardingPageMsg::Connect,

                    #[watch]
                    set_sensitive: !model.connecting,
                    #[wrap(Some)]
                    set_child = if model.connecting {
                        adw::Spinner {}
                    } else {
                        gtk::Label {
                            set_label: &gettext("Connect"),
                        }
                    }
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
                    set_css_classes: &["suggested-action", "pill"],
                    connect_clicked => OnboardingPageMsg::NextPage,
                }
            }
        },
        spinner = &adw::Spinner {}
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let history_scale_adjustment = gtk::Adjustment::builder()
            .upper(100.0)
            .lower(0.0)
            .step_increment(0.1)
            .build();

        let url_focus_controller = gtk::EventControllerFocus::new();
        let focus_sender = sender.clone();
        url_focus_controller.connect_leave(move |_| {
            focus_sender.input(OnboardingPageMsg::UrlEntered);
        });

        let model = Self {
            step: OnboardingStep::Welcome,
            transition: gtk::StackTransitionType::SlideLeft,
            sync_all: false,
            url_error: None,
            password_error: None,
            url: String::new(),
            password: String::new(),
            connecting: false,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
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
            OnboardingPageMsg::UrlChanged(url) => {
                self.url = url;
                if ALLOWED_URL_CHARS_REGEX.is_match(&self.url).unwrap_or(false) {
                    self.url_error = None;
                } else {
                    self.url_error = Some(gettext("Invalid URL"));
                }
            }
            OnboardingPageMsg::UrlEntered => {
                println!("UrlEntered called with url: {}", self.url);
                if URL_REGEX.is_match(&self.url).unwrap_or(false) {
                    self.url_error = None;
                } else {
                    self.url_error = Some(gettext("Invalid URL"));
                }
            }
            OnboardingPageMsg::PasswordChanged(password) => {
                self.password = password;
                self.password_error = None;
            }
            OnboardingPageMsg::Connect => {
                if let Some(url_error) = self.url_error.clone() {
                    APP_BROKER.send(AppMsg::ShowToast(url_error));
                } else {
                    self.connecting = true;

                    // try to connect to the server

                    // Simulate an invalid password
                    if true {
                        self.password_error = Some(gettext("Invalid credentials"));
                        APP_BROKER.send(AppMsg::ShowToast(gettext("Invalid credentials")));
                    }

                    // sender.input(OnboardingPageMsg::NextPage);

                    self.connecting = false;
                }
            }
            _ => (),
        }
    }
}
