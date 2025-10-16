/* conversation-row.vala
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

[GtkTemplate (ui = "/org/elliotnash/TuxBubbles/widgets/conversation-row.ui")]
public class TuxBubbles.ConversationRow : Adw.Bin {
    [GtkChild]
    private unowned Adw.Avatar avatar;
    [GtkChild]
    private unowned Gtk.Label name_label;
    [GtkChild]
    private unowned Gtk.Label time_label;
    [GtkChild]
    private unowned Gtk.Label message_label;
    [GtkChild]
    private unowned Gtk.Label unread_label;

    private TuxBubbles.Chat? _chat = null;
    public TuxBubbles.Chat? chat {
        get { return _chat; }
        set {
            _chat = value;
            update_display ();
        }
    }

    private void update_display () {
        if (chat == null) {
            name_label.set_text ("");
            time_label.set_text ("");
            message_label.set_text ("");
            unread_label.visible = false;
            return;
        }

        name_label.set_text (chat.get_effective_display_name ());
        time_label.set_text (chat.get_timestamp_string ());
        message_label.set_text (chat.get_last_message_preview ());
        
        if (chat.unread_count > 0) {
            unread_label.set_text (chat.unread_count.to_string ());
            unread_label.visible = true;
        } else {
            unread_label.visible = false;
        }

        // Set avatar text to first character of display name
        var display_name = chat.get_effective_display_name ();
        if (display_name.length > 0) {
            avatar.set_text (display_name.substring (0, 1).up ());
        } else {
            avatar.set_text ("?");
        }
    }
}