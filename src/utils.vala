public class TuxBubbles.Utils {
    public static void show_toast(owned Adw.Toast toast) {
        var app = (TuxBubbles.Application?) GLib.Application.get_default();
        var window = (TuxBubbles.Window?) app?.get_active_window();
        window?.toast_overlay.add_toast(toast);
    }
}