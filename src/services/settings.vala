public class TuxBubbles.Settings : GLib.Object {
    private static Settings? _instance = null;
    public static Settings instance {
        get {
            if (_instance == null) {
                _instance = new Settings();
            }
            return _instance;
        }
    }

    private GLib.Settings settings;
    private Secret.Service? secret_service = null;

    public signal void server_url_changed (string url);
    public signal void compact_mode_changed (bool compact);

    private Settings () {
        settings = new GLib.Settings("org.elliotnash.TuxBubbles");
        
        // Connect to GSettings changes
        settings.changed["server-url"].connect (() => {
            server_url_changed (settings.get_string ("server-url"));
        });
        
        settings.changed["compact-mode"].connect (() => {
            compact_mode_changed (settings.get_boolean ("compact-mode"));
        });

        // Initialize Secret Service
        init_secret_service.begin ();
    }

    private async void init_secret_service () {
        try {
            secret_service = yield Secret.Service.get (Secret.ServiceFlags.NONE, null);
        } catch (Error e) {
            warning ("Failed to initialize Secret Service: %s", e.message);
        }
    }

    // GSettings properties
    public string server_url {
        owned get { return settings.get_string ("server-url"); }
        set { settings.set_string ("server-url", value); }
    }

    public bool compact_mode {
        get { return settings.get_boolean ("compact-mode"); }
        set { settings.set_boolean ("compact-mode", value); }
    }

    // Secret Service methods for password storage
    public async bool store_password (string password) {
        if (secret_service == null) {
            warning ("Secret Service not available");
            return false;
        }

        try {
            var attributes = new HashTable<string, string> (str_hash, str_equal);
            attributes["account"] = "default";
            attributes["service"] = "org.elliotnash.TuxBubbles";

            var schema = new Secret.Schema ("org.elliotnash.TuxBubbles", Secret.SchemaFlags.NONE,
                "account", Secret.SchemaAttributeType.STRING,
                "service", Secret.SchemaAttributeType.STRING);

            yield Secret.password_storev (
                schema,
                attributes,
                Secret.COLLECTION_DEFAULT,
                "BlueBubbles Server Password",
                password,
                null
            );
            return true;
        } catch (Error e) {
            warning ("Failed to store password: %s", e.message);
            return false;
        }
    }

    public async string? retrieve_password () {
        if (secret_service == null) {
            warning ("Secret Service not available");
            return null;
        }

        try {
            var attributes = new HashTable<string, string> (str_hash, str_equal);
            attributes["account"] = "default";
            attributes["service"] = "org.elliotnash.TuxBubbles";

            var schema = new Secret.Schema ("org.elliotnash.TuxBubbles", Secret.SchemaFlags.NONE,
                "account", Secret.SchemaAttributeType.STRING,
                "service", Secret.SchemaAttributeType.STRING);

            var password = yield Secret.password_lookupv (
                schema,
                attributes,
                null
            );
            return password;
        } catch (Error e) {
            warning ("Failed to retrieve password: %s", e.message);
            return null;
        }
    }

    public async bool delete_password () {
        if (secret_service == null) {
            warning ("Secret Service not available");
            return false;
        }

        try {
            var attributes = new HashTable<string, string> (str_hash, str_equal);
            attributes["account"] = "default";
            attributes["service"] = "org.elliotnash.TuxBubbles";

            var schema = new Secret.Schema ("org.elliotnash.TuxBubbles", Secret.SchemaFlags.NONE,
                "account", Secret.SchemaAttributeType.STRING,
                "service", Secret.SchemaAttributeType.STRING);

            yield Secret.password_clearv (
                schema,
                attributes,
                null
            );
            return true;
        } catch (Error e) {
            warning ("Failed to delete password: %s", e.message);
            return false;
        }
    }

    public bool has_stored_password () {
        if (secret_service == null) {
            return false;
        }

        try {
            var attributes = new HashTable<string, string> (str_hash, str_equal);
            attributes["account"] = "default";
            attributes["service"] = "org.elliotnash.TuxBubbles";

            var schema = new Secret.Schema ("org.elliotnash.TuxBubbles", Secret.SchemaFlags.NONE,
                "account", Secret.SchemaAttributeType.STRING,
                "service", Secret.SchemaAttributeType.STRING);

            var password = Secret.password_lookupv_sync (
                schema,
                attributes,
                null
            );
            return password != null;
        } catch (Error e) {
            return false;
        }
    }
}