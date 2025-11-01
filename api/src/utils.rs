/// Builds a vector of string literals for variables that are `true`.
///
/// This macro takes key-value pairs where the key is a boolean variable
/// and the value is a string literal. It returns a `Vec<String>` containing
/// only the values whose corresponding boolean variables are `true`.
///
/// # Usage
///
/// ```rust,ignore
/// let with_last_message = true;
/// let with_participants = false;
/// let with_sms = true;
///
/// let params = build_option_list! {
///     with_last_message => "lastmessage",
///     with_participants => "participants",
///     with_sms => "sms",
/// }.join(",");
///
/// // params == "lastmessage,sms"
/// ```
///
/// This is particularly useful for building HTTP query parameters where
/// you only want to include parameters that are enabled.
macro_rules! build_option_list {
    (@count) => { 0 };
    (@count $_key:ident => $_var:literal) => { 1 };
    (@count $_key:ident => $_var:literal, $($rest_key:ident => $rest_var:literal),* $(,)?) => {
        1 + build_option_list!(@count $($rest_key => $rest_var),*)
    };
    ($($key:ident => $var:literal),* $(,)?) => {{
        let mut with = Vec::with_capacity(build_option_list!(@count $($key => $var),*));
        $(
            if $key {
                with.push($var);
            }
        )*
        with
    }};
}
pub(crate) use build_option_list;
