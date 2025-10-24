use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    adw::{self, prelude::AdwDialogExt},
};

pub struct ShortcutsDialog {
    parent: Option<adw::ApplicationWindow>,
}

impl SimpleComponent for ShortcutsDialog {
    type Init = Option<adw::ApplicationWindow>;
    type Input = ();
    type Output = ();
    type Widgets = adw::ShortcutsDialog;
    type Root = adw::ShortcutsDialog;

    fn init_root() -> Self::Root {
        adw::ShortcutsDialog::new()
    }

    fn init(
        parent: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { parent };
        let widgets = root.clone();

        // Create shortcuts section
        let general_section = adw::ShortcutsSection::new(Some("General"));

        // Add "Show Shortcuts" shortcut
        let show_shortcuts_item = adw::ShortcutsItem::new("Show Shortcuts", "<Primary>question");
        general_section.add(show_shortcuts_item);

        // Add "Quit" shortcut
        let quit_item = adw::ShortcutsItem::new("Quit", "<Primary>q");
        general_section.add(quit_item);

        // Add the section to the dialog
        widgets.add(general_section);

        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.present(self.parent.as_ref());
    }
}
