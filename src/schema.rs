// @generated automatically by Diesel CLI.

diesel::table! {
    selectors (id) {
        id -> Uuid,
        function_name -> Varchar,
        felt_selector -> Text,
        selector -> Varchar,
    }
}
