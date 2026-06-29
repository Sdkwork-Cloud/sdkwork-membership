use sdkwork_contract_service::CommerceServiceError;
use sdkwork_utils_rust::string;

/// Validate that a field value is non-empty after trimming.
pub fn require_non_empty(field: &str, value: &str) -> Result<(), CommerceServiceError> {
    if string::is_blank(Some(value)) {
        return Err(CommerceServiceError::validation(format!(
            "{field} is required"
        )));
    }

    Ok(())
}

/// Normalize an optional text value: trim whitespace, return None for empty.
/// Centralized here to eliminate duplication across domain and query modules.
pub fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    if string::is_blank(value) {
        return None;
    }
    Some(value.unwrap().trim().to_string())
}
