/* conversation-list-page.vala */

[GtkTemplate (ui = "/org/elliotnash/TuxBubbles/pages/conversation-list-page.ui")]
public class TuxBubbles.ConversationListPage : Adw.Bin {
    public signal void chat_selected (string chat_id);

    public ConversationListPage () {
        Object ();
    }
}


