/* conversation-list-page.vala
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

[GtkTemplate (ui = "/org/elliotnash/TuxBubbles/pages/conversation-list-page.ui")]
public class TuxBubbles.ConversationListPage : Adw.NavigationPage {
    [GtkChild]
    private unowned Gtk.ListView list_view;

    public signal void chat_selected (string chat_id);

    private Gtk.SingleSelection selection_model;
    private GLib.ListStore list_store;

    public ConversationListPage () {
        Object ();
    }

    construct {
        setup_list_view ();
        load_mock_data ();
    }

    private void setup_list_view () {
        // Create list store and selection model
        list_store = new GLib.ListStore (typeof (TuxBubbles.Chat));
        selection_model = new Gtk.SingleSelection (list_store);
        list_view.set_model (selection_model);

        // Create factory for list items
        var factory = new Gtk.SignalListItemFactory ();
        factory.setup.connect (on_setup_item);
        factory.bind.connect (on_bind_item);
        list_view.set_factory (factory);

        // Connect selection changes
        selection_model.selection_changed.connect (on_selection_changed);
    }

    private void on_setup_item (Gtk.SignalListItemFactory factory, GLib.Object object) {
        var list_item = object as Gtk.ListItem;
        if (list_item != null) {
            var row = new TuxBubbles.ConversationRow ();
            list_item.set_child (row);
        }
    }

    private void on_bind_item (Gtk.SignalListItemFactory factory, GLib.Object object) {
        var list_item = object as Gtk.ListItem;
        if (list_item != null) {
            var row = list_item.get_child () as TuxBubbles.ConversationRow;
            var chat = list_item.get_item () as TuxBubbles.Chat;
            if (row != null && chat != null) {
                row.chat = chat;
            }
        }
    }

    private void on_selection_changed (uint position, uint n_items) {
        if (position != Gtk.INVALID_LIST_POSITION) {
            var chat = list_store.get_item (position) as TuxBubbles.Chat;
            if (chat != null) {
                chat_selected (chat.guid);
            }
        }
    }

    private void load_mock_data () {
        // Clear existing data
        list_store.remove_all ();

        // Add mock chats based on the JSON structure
        var now = new DateTime.now_utc ();
        var one_hour_ago = now.add_hours (-1).to_unix () * 1000;
        var yesterday = now.add_days (-1).to_unix () * 1000;
        var last_week = now.add_days (-7).to_unix () * 1000;

        var chats = new TuxBubbles.Chat[] {
            new TuxBubbles.Chat (
                3, "iMessage;-;+15412078154", 45, "+15412078154",
                false, "", false, "E71889AA-F44B-41E6-A899-EEBCF46D8955",
                "+15412311769", "Ohh haha", one_hour_ago, false, 2
            ),
            new TuxBubbles.Chat (
                4, "iMessage;-;+15551234567", 45, "+15551234567",
                false, "John Smith", false, "F1234567-1234-5678-9ABC-123456789DEF",
                "+15551234567", "Thanks for the update!", yesterday, true, 0
            ),
            new TuxBubbles.Chat (
                5, "iMessage;-;+15559876543", 45, "+15559876543",
                false, "Sarah Johnson", false, "G2345678-2345-6789-ABCD-234567890EFG",
                "+15559876543", "See you tomorrow", last_week, false, 1
            ),
            new TuxBubbles.Chat (
                6, "iMessage;-;+15555555555", 45, "+15555555555",
                false, "Mom", false, "H3456789-3456-789A-BCDE-345678901FGH",
                "+15555555555", "Call me when you get home", one_hour_ago, false, 0
            ),
            new TuxBubbles.Chat (
                7, "iMessage;-;+15551111111", 45, "+15551111111",
                false, "Work Group", false, "I4567890-4567-89AB-CDEF-456789012GHI",
                "+15551111111", "Meeting moved to 3pm", yesterday, true, 3
            )
        };

        foreach (var chat in chats) {
            list_store.append (chat);
        }
    }
}


