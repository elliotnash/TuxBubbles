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
    CONNECTION = 1;

    public string get_name() {
        // Remove namespace/enum name
        return this.to_string().down().substring(28);
    }

    public OnboardingStep get_next() {
        if (this != CONNECTION) {
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

    private void next_page() {
        current_step = current_step.get_next();
        onboarding_stack.transition_type = Gtk.StackTransitionType.SLIDE_LEFT;
        onboarding_stack.visible_child_name = current_step.get_name();
        previous_btn_revealer.reveal_child = true;
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
        string url = server_url_entry.text;
        string password = server_password_entry.text;
        print("Url=%s, Password=%s\n", url, password);

        if (!urlRegex.match(server_url_entry.text)) {
            // TODO: send toast or banner or smth
            return;
        }

        // Start ping
        connect_btn.sensitive = false;
        connect_btn.child = connect_btn_spinner;

        // Test connection
        yield wait(1000);

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
