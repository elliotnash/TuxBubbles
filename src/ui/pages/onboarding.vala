/* window.vala
 *
 * Copyright 2025 Unknown
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

enum TuxBubbles.OnboardingStep {
    WELCOME = 0,
    CONNECTION = 1,
    SYNC = 2;

    public string get_name() {
        // Remove namespace/enum name
        return this.to_string().down().substring(28);
    }

    public OnboardingStep get_next() {
        if (this != SYNC) {
            return this + 1;
        }
        return this;
    }

    public OnboardingStep get_previous() {
        if (this != WELCOME) {
            return this - 1;
        }
        return this;
    }
}

[GtkTemplate (ui = "/org/elliotnash/TuxBubbles/pages/onboarding.ui")]
public class TuxBubbles.Onboarding : Adw.Bin {
    [GtkChild]
    private unowned Gtk.Stack onboarding_stack;
    [GtkChild]
    private unowned Gtk.Button welcome_continue_btn;
    [GtkChild]
    private unowned Gtk.Button previous_btn;
    [GtkChild]
    private unowned Gtk.Revealer previous_btn_revealer;
    [GtkChild]
    private unowned Adw.EntryRow server_url_entry;
    [GtkChild]
    private unowned Adw.PasswordEntryRow server_password_entry;
    [GtkChild]
    private unowned Gtk.Button connect_btn;

    private Adw.Spinner connect_btn_spinner = new Adw.Spinner();
    private Gtk.EventControllerFocus server_url_entry_focus_controller = new Gtk.EventControllerFocus();

    private OnboardingStep current_step = OnboardingStep.WELCOME;

    Regex urlRegex = /^(https?:\/\/)(([a-zA-Z0-9](?:(?:[a-zA-Z0-9-]*|(?<!-)\.(?![-.]))*[a-zA-Z0-9]+)?))(:(\d+))?$/;
    Regex allowedUrlCharsRegex  = /^[a-zA-Z0-9.\-:\/]*$/;

    private async void next_page() {
        print("Querying chats\n");
        try {
        var res = yield APIClient.instance.chat_query();
            if (res.is_success()) {
                print(res.status.to_string() + "\n");
                print("Chat query successful, %d chats found\n", res.data.size);
                foreach (var chat in res.data) {
                    print("Chat: %s\n", chat.guid);
                    print("Chat: %s\n", chat.display_name);
                }
            } else {
                print("API Error: %s\n", res.error.message);
            }
        } catch (Error e) {
            print("Error querying chats: %s\n", e.message);
        }
        //  current_step = current_step.get_next();
        //  onboarding_stack.transition_type = Gtk.StackTransitionType.SLIDE_LEFT;
        //  onboarding_stack.visible_child_name = current_step.get_name();
        //  previous_btn_revealer.reveal_child = true;
    }

    private void previous_page() {
        current_step = current_step.get_previous();
        onboarding_stack.transition_type = Gtk.StackTransitionType.SLIDE_RIGHT;
        onboarding_stack.visible_child_name = current_step.get_name();
        if (current_step == OnboardingStep.WELCOME) {
            previous_btn_revealer.reveal_child = false;
        }
    }

    private void server_url_changed() {
        // If all characters are valid, remove error class, otherwise add it.
        if (allowedUrlCharsRegex.match(server_url_entry.text)) {
            server_url_entry.set_tooltip_text(null);
            server_url_entry.remove_css_class("error");
        } else {
            server_url_entry.set_tooltip_text("Invalid URL");
            server_url_entry.add_css_class("error");
        }
    }

    private void server_url_focus_lost() {
        if (urlRegex.match(server_url_entry.text)) {
            server_url_entry.set_tooltip_text(null);
            server_url_entry.remove_css_class("error");
        } else {
            server_url_entry.set_tooltip_text("Invalid URL");
            server_url_entry.add_css_class("error");
        }
    }

    public async void wait(uint interval) {
        GLib.Timeout.add(interval, () => {
            wait.callback();
            return false;
        });
        yield;
    }

    private async void server_connect() {
        if (!urlRegex.match(server_url_entry.text)) {
            return;
        }

        // Start ping
        connect_btn.sensitive = false;
        connect_btn.child = connect_btn_spinner;

        // Set credentials
        Settings.instance.server_url = server_url_entry.text;
        yield Settings.instance.store_password(server_password_entry.text);

        // Test connection
        try {
            var response = yield APIClient.instance.ping();
            if (response.is_success()) {
                print("BlueBubbles server connection successful\n");
                next_page();
            } else {
                print("API Error: %s\n", response.error.message);
                var message = response.status == Soup.Status.UNAUTHORIZED ? N_("Invalid credentials") : response.error?.message;
                var toast = new Adw.Toast("<span foreground=\"red\">" + _(message) + "</span>");
                Utils.show_toast(toast);
            }
        } catch (Error e) {
            print("Error connecting: %s\n", e.message);
            var toast = new Adw.Toast("<span foreground=\"red\">" + _("Failed to connect to server") + "</span>");
            Utils.show_toast(toast);
        }
        
        // Restore button
        connect_btn.label = _("Connect");
        connect_btn.sensitive = true;
    }

    construct {
        server_url_entry.add_controller(server_url_entry_focus_controller);

        previous_btn.clicked.connect(this.previous_page);
        welcome_continue_btn.clicked.connect(this.next_page);

        server_url_entry.notify["text"].connect(this.server_url_changed);
        server_url_entry_focus_controller.leave.connect(this.server_url_focus_lost);
        server_url_entry.entry_activated.connect(() => {
            server_password_entry.grab_focus();
        });

        server_password_entry.entry_activated.connect(this.server_connect);
        connect_btn.clicked.connect(this.server_connect);
    }
}
