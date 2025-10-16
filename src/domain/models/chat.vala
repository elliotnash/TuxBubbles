/* chat.vala
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

public class TuxBubbles.Chat : GLib.Object {
    public int original_rowid { get; set; }
    public string guid { get; set; }
    public int style { get; set; }
    public string chat_identifier { get; set; }
    public bool is_archived { get; set; }
    public string display_name { get; set; }
    public bool is_filtered { get; set; }
    public string group_id { get; set; }
    public string last_addressed_handle { get; set; }
    
    // Last message data (simplified for now)
    public string last_message_text { get; set; }
    public int64 last_message_date { get; set; }
    public bool last_message_is_from_me { get; set; }
    public int unread_count { get; set; }

    public Chat (int original_rowid, string guid, int style, string chat_identifier,
                 bool is_archived, string display_name, bool is_filtered, string group_id,
                 string last_addressed_handle, string last_message_text, int64 last_message_date,
                 bool last_message_is_from_me, int unread_count) {
        this.original_rowid = original_rowid;
        this.guid = guid;
        this.style = style;
        this.chat_identifier = chat_identifier;
        this.is_archived = is_archived;
        this.display_name = display_name;
        this.is_filtered = is_filtered;
        this.group_id = group_id;
        this.last_addressed_handle = last_addressed_handle;
        this.last_message_text = last_message_text;
        this.last_message_date = last_message_date;
        this.last_message_is_from_me = last_message_is_from_me;
        this.unread_count = unread_count;
    }

    public string get_effective_display_name () {
        if (display_name != null && display_name != "") {
            return display_name;
        }
        return chat_identifier;
    }

    public string get_last_message_preview () {
        if (last_message_text != null && last_message_text != "") {
            return last_message_text;
        }
        return _("No messages");
    }

    public string get_timestamp_string () {
        var date = new DateTime.from_unix_utc (last_message_date / 1000);
        var now = new DateTime.now_utc ();
        var diff = now.difference (date);
        
        if (diff < TimeSpan.DAY) {
            return date.format ("%H:%M");
        } else if (diff < TimeSpan.DAY * 7) {
            return date.format ("%a");
        } else {
            return date.format ("%m/%d");
        }
    }
}
