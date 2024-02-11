#[macro_export]
macro_rules! hashmap {
    // Declarative macros look similar to match arms. Here, the arm is empty, so
    // return an empty hashmap.
    () => {
        {
            ::std::collections::HashMap::new()
        }
    };

    // Expect one or more key-value pairs in this arm. Note a trailing comma is
    // optional.
    ( $( $key:expr => $val:expr),+ $(,)? ) => {{
        use ::std::collections::HashMap;

        let mut hm = HashMap::new();

        $(
            hm.insert($key, $val);
        )*

        hm
    }};
}
