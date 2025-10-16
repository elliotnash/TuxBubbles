/* message-composer.vala */

[GtkTemplate (ui = "/org/elliotnash/TuxBubbles/widgets/message-composer.ui")]
public class TuxBubbles.MessageComposer : Adw.Bin {
    public signal void send_text (string text);
}

