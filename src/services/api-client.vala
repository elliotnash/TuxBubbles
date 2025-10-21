
public class TuxBubbles.APIClient : GLib.Object {
    private static APIClient? _instance = null;
    public static APIClient instance {
        get {
            if (_instance == null) {
                _instance = new APIClient ();
            }
            return _instance;
        }
    }

    private Soup.Session session;
    private TuxBubbles.Settings settings;

    private APIClient () {
        session = new Soup.Session ();
        settings = TuxBubbles.Settings.instance;
    }

    public delegate T? DataParser<T>(Json.Node node);

    // Generic method to make API calls and parse responses
    private async APIResponse<TRes>? make_request<TRes> (string endpoint, DataParser<TRes> parse_data, string method = "GET", GLib.Object? payload = null, Gee.Map<string, string>? query_params = new Gee.HashMap<string, string?>()) throws Error {
        var server_url = settings.server_url;
        if (server_url == "") {
            throw new IOError.INVALID_ARGUMENT ("Server URL not configured");
        }

        var password = yield settings.retrieve_password();
        if (password == null) {
            throw new IOError.INVALID_ARGUMENT ("Password not configured");
        }

        // Add password to query params but don't overwrite if it already exists
        if (!query_params.has_key("password")) {
            query_params.set("password", password);
        }

        var query_string = "";
        foreach (var entry in query_params) {
            query_string += "%s=%s&".printf(GLib.Uri.escape_string(entry.key), GLib.Uri.escape_string(entry.value));
        }
        query_string = query_string.substring(0, query_string.length - 1);

        var url = "%s%s?%s".printf(server_url, endpoint, query_string);
        
        var message = new Soup.Message(method, url);

        // Set headers
        message.request_headers.append("Content-Type", "application/json");
        message.request_headers.append("User-Agent", "TuxBubbles/1.0");

        // Add payload for POST/PUT requests
        if (payload != null && (method == "POST" || method == "PUT")) {
            var generator = new Json.Generator();
            generator.set_root(Json.gobject_serialize(payload));
            var json_string = generator.to_data(null);
            message.set_request_body_from_bytes("application/json", new Bytes.take(json_string.data));
        }
        
        var response_data = yield session.send_and_read_async(message, Priority.DEFAULT, null);
      
        // Parse JSON response
        var parser = new Json.Parser();
        parser.load_from_data((string) response_data.get_data(), -1);

        var root = parser.get_root();
        if (root == null) {
            // It's expected that a some non 200 responses are not valid JSON, so throw the HTTP error.
            // If a 200 response is not valid JSON, that's a problem.
            if (message.status_code < 200 || message.status_code >= 300) {
                throw new IOError.FAILED("HTTP %u: %s".printf (message.status_code, message.reason_phrase));
            } else {
                throw new IOError.FAILED("Invalid JSON response");
            }
        }

        var root_object = root.get_object();
        if (root_object == null) {
            throw new IOError.FAILED("Response is not a JSON object");
        }

        // Extract standard response fields
        var status = (int) root_object.get_int_member("status");
        var message_text = root_object.get_string_member("message");
        var data_node = root_object.get_member("data");
        var error_node = root_object.get_member("error");

        // Parse the data field based on type T
        TRes? data = null;
        if (data_node != null && !data_node.is_null()) {
            data = parse_data<TRes> (data_node);
        }

        APIError? error = null;
        if (error_node != null && !error_node.is_null()) {
            error = parse_error (error_node);
        }

        return new APIResponse<TRes> (status, message_text, data, error);
    }

    private APIError? parse_error (Json.Node error_node) {
        var error_object = error_node.get_object();
        if (error_object == null) {
            return null;
        }
        var error_type = error_object.get_string_member("type");
        var message = error_object.get_string_member("message");
        return new APIError(error_type, message);
    }

    // Type-specific data parsing
    private T? parse_object<T> (Json.Node data_node) {
        if (typeof (T) == typeof(string)) {
            return (T) data_node.get_string();
        //  } else if (typeof (T) == typeof(int)) {
        //      return (T) data_node.get_int();
        //  } else if (typeof (T) == typeof(double)) {
        //      return (T) (double) data_node.get_double();
        //  } else if (typeof (T) == typeof(bool)) {
        //      return (T) (bool) data_node.get_boolean();
        } else {
            return Json.gobject_deserialize(typeof (T), data_node);
        }
    }

    private Gee.List<T>? parse_data_array<T> (Json.Node data_node) {
        var array = new Gee.ArrayList<T>();
        var array_node = data_node.get_array();
        if (array_node == null) {
            return null;
        }
        foreach (var node in array_node.get_elements()) {
            array.add(parse_object<T>(node));
        }
        return array;
    }

    // Helper method to check if server is reachable
    public async bool is_server_reachable() {
        try {
            var response = yield ping();
            return response != null && response.is_success() && response.data == "pong";
        } catch (Error e) {
            warning("Server unreachable: %s", e.message);
            return false;
        }
    }

    // Ping endpoint
    public async APIResponse<string>? ping() throws Error {
        return yield make_request<string>("/api/v1/ping", (node) => parse_object<string>(node));
    }

    public async APIResponse<Gee.List<Chat>>? chat_query() throws Error {
        return yield make_request<Gee.List<Chat>>("/api/v1/chat/query", (node) => parse_data_array<Chat>(node), "POST", new ChatQueryRequest());
    }
}

class TuxBubbles.ChatQueryRequest : GLib.Object {
    public int? limit { get; set; }
    public int? offset { get; set; }
    public Gee.List<string>? with { get; set; }
    public string? sort { get; set; }

    public ChatQueryRequest(int? limit = null, int? offset = null, Gee.List<string>? with = null, string? sort = null) {
        this.limit = limit;
        this.offset = offset;
        this.with = with;
        this.sort = sort;
    }
}
