// @generated automatically by Diesel CLI.

diesel::table! {
    daily_logs (id) {
        id -> Text,
        content -> Text,
        log_date -> Text,
    }
}
