
public class TuxBubbles.APIResponse<T> : GLib.Object {
    public int status { get; set; }
    public string message { get; set; }
    public T? data { get; set; }
    public APIError? error { get; set; }


    public APIResponse (int status, string message, T? data, APIError? error) {
        this.status = status;
        this.message = message;
        this.data = data;
        this.error = error;
    }

    public bool is_success () {
        return (status >= 200 && status < 300);
    }

    public bool is_error () {
        return (error != null);
    }
}

public class TuxBubbles.APIError : GLib.Object {
    public string error_type { get; set; }
    public string message { get; set; }

    public APIError (string error_type, string message) {
        this.error_type = error_type;
        this.message = message;
    }
}
