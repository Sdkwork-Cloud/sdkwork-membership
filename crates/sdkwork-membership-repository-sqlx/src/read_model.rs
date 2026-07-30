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
