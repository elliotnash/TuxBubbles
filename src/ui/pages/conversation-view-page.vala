/* conversation-view-page.vala */

[GtkTemplate (ui = "/org/elliotnash/TuxBubbles/pages/conversation-view-page.ui")]
public class TuxBubbles.ConversationViewPage : Adw.NavigationPage {
    static construct {
        typeof(TuxBubbles.MessageComposer).ensure ();
    }

    construct {
        //  composer = new TuxBubbles.MessageComposer ();
        //  toolbar_view.add_bottom_bar (composer);
    }

    public void load_chat (string chat_id) {
    }
}
