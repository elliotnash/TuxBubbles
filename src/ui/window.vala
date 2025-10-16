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

[GtkTemplate (ui = "/org/elliotnash/TuxBubbles/window.ui")]
public class TuxBubbles.Window : Adw.ApplicationWindow {
	[GtkChild]
	private unowned TuxBubbles.ConversationListPage sidebar_page;
	[GtkChild]
	private unowned TuxBubbles.ConversationViewPage content_page;

	public Window (Gtk.Application app) {
		Object (application: app);
	}

	static construct {
        typeof(TuxBubbles.ConversationListPage).ensure ();
        typeof(TuxBubbles.ConversationViewPage).ensure ();
    }

	construct {
		//  list_page.chat_selected.connect ((chat_id) => {
		//  	view_page.load_chat (chat_id);
		//  });
	}
}
