use std::{fmt, sync::LazyLock};

use fancy_regex::Regex;
use gettextrs::gettext;
use libadwaita::prelude::{EntryRowExt, NavigationPageExt, PreferencesRowExt};
use relm4::{
    Component, ComponentParts, ComponentSender, RelmWidgetExt, WidgetTemplate,
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
}

#[derive(Debug, PartialEq)]
pub enum OnboardingPageMsg {
    NextPage,
    SyncStepFromTag(String),
    ToggleSyncAll,
    UrlChanged(String),
    UrlEntered,
    PasswordChanged(String),
    Connect,
}

pub struct OnboardingPage {
    step: OnboardingStep,
    url: String,
    url_error: Option<String>,
    password: String,
    password_error: Option<String>,
    connecting: bool,
    sync_all: bool,
}

#[relm4::widget_template(pub)]
impl WidgetTemplate for OnboardingToolbar {
    view! {
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                pack_end = &gtk::Button {
                    set_icon_name: "help-about-symbolic",
                    set_action_name: Some(&AboutAction::action_name()),
                    set_tooltip_text: Some(&gettext("About TuxBubbles"))
                }
            },
        }
    }
}

#[relm4::component(pub)]
impl Component for OnboardingPage {
    type Input = OnboardingPageMsg;
    type Output = ();
    type Init = ();
    type CommandOutput = ();

    view! {
        // #[root]
        // adw::ToolbarView {
        //     add_top_bar = &adw::HeaderBar {
        //         pack_end = &gtk::Button {
        //             set_icon_name: "help-about-symbolic",
        //             set_action_name: Some(&AboutAction::action_name()),
        //             set_tooltip_text: Some(&gettext("About TuxBubbles"))
        //         }
        //     },

        //     set_content: Some(&navigation_view)

        // },
        // navigation_view = &adw::NavigationView {
        //     add: &welcome_page,
        //     add: &connection_page,
        //     add: &sync_page,
        // },
        #[root]
        navigation_view = &adw::NavigationView {
            add: &welcome_page,
            add: &connection_page,
            add: &sync_page,
        },
        welcome_page = &adw::NavigationPage {
            set_title: &gettext("Welcome to TuxBubbles"),
            set_tag: Some(&OnboardingStep::Welcome.to_string()),

            #[template]
            OnboardingToolbar {
                set_content: Some(&welcome)
            }
        },
        connection_page = &adw::NavigationPage {
            set_title: &gettext("Connect to your BlueBubbles instance"),
            set_tag: Some(&OnboardingStep::Connection.to_string()),

            #[template]
            OnboardingToolbar {
                set_content: Some(&connection)
            }
        },
        sync_page = &adw::NavigationPage {
            set_title: &gettext("Sync your message"),
            set_tag: Some(&OnboardingStep::Sync.to_string()),

            #[template]
            OnboardingToolbar {
                set_content: Some(&sync)
            }
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
            sync_all: false,
            url_error: None,
            password_error: None,
            url: String::new(),
            password: String::new(),
            connecting: false,
        };
        let widgets = view_output!();

        // Initialize NavigationView with the welcome page
        root.push_by_tag(&OnboardingStep::Welcome.to_string());

        // Connect to the popped signal to update step when back button is clicked
        let nav_sender = sender.clone();
        widgets
            .navigation_view
            .connect_popped(move |nav_view, _popped_page| {
                // Get the currently visible page after the pop and sync step accordingly
                if let Some(visible_page) = nav_view.visible_page() {
                    if let Some(tag) = visible_page.tag() {
                        nav_sender
                            .input(OnboardingPageMsg::SyncStepFromTag(tag.as_str().to_string()));
                    }
                }
            });

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        // Handle navigation and model updates together
        match message {
            OnboardingPageMsg::NextPage => {
                self.step = self.step.next();
                root.push_by_tag(&self.step.to_string());
            }
            OnboardingPageMsg::SyncStepFromTag(tag) => {
                // Sync step with the visible page's tag (used when back button is clicked)
                if tag == OnboardingStep::Welcome.to_string() {
                    self.step = OnboardingStep::Welcome;
                } else if tag == OnboardingStep::Connection.to_string() {
                    self.step = OnboardingStep::Connection;
                } else if tag == OnboardingStep::Sync.to_string() {
                    self.step = OnboardingStep::Sync;
                }
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
                    if false {
                        self.password_error = Some(gettext("Invalid credentials"));
                        APP_BROKER.send(AppMsg::ShowToast(gettext("Invalid credentials")));
                    } else {
                        sender.input(OnboardingPageMsg::NextPage);
                    }

                    self.connecting = false;
                }
            }
        }

        // Update the view
        self.update_view(widgets, sender);
    }
}
