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
	private unowned Adw.NavigationPage sidebar_page;
	[GtkChild]
	private unowned Adw.NavigationPage content_page;

	private TuxBubbles.ConversationListPage list_page;
	private TuxBubbles.ConversationViewPage view_page;

	public Window (Gtk.Application app) {
		Object (application: app);
	}

	construct {
		list_page = new TuxBubbles.ConversationListPage ();
		view_page = new TuxBubbles.ConversationViewPage ();

		// Set the pages as children of the NavigationPage objects
		sidebar_page.set_child (list_page);
		content_page.set_child (view_page);

		list_page.chat_selected.connect ((chat_id) => {
			view_page.load_chat (chat_id);
		});
	}
}
