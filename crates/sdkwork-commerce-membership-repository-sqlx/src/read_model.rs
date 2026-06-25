pub(crate) fn is_missing_sqlite_read_model(error: &sqlx::Error) -> bool {
    if matches!(error, sqlx::Error::ColumnNotFound(_)) {
        return true;
    }
    error
        .as_database_error()
        .map(|database_error| {
            let message = database_error.message().to_ascii_lowercase();
            message.contains("no such table") || message.contains("no such column")
        })
        .unwrap_or(false)
}

pub(crate) fn is_missing_postgres_read_model(error: &sqlx::Error) -> bool {
    if matches!(error, sqlx::Error::ColumnNotFound(_)) {
        return true;
    }
    error
        .as_database_error()
        .and_then(|database_error| database_error.code())
        .map(|code| matches!(code.as_ref(), "42P01" | "42703"))
        .unwrap_or(false)
}
