/* conversation-view-page.vala */

[GtkTemplate (ui = "/org/elliotnash/TuxBubbles/pages/conversation-view-page.ui")]
public class TuxBubbles.ConversationViewPage : Adw.Bin {
    [GtkChild]
    private unowned Adw.ToolbarView toolbar_view;

    private TuxBubbles.MessageComposer composer;

    construct {
        composer = new TuxBubbles.MessageComposer ();
        toolbar_view.add_bottom_bar (composer);
    }

    public void load_chat (string chat_id) {
    }
}


