use axum::Extension;
use sdkwork_iam_context_service::IamAppContext;
use sdkwork_utils_rust::number::parse_int;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NumericRuntimeSubject {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
}

pub(crate) fn numeric_runtime_subject_from_extension(
    context: Option<Extension<IamAppContext>>,
) -> Result<NumericRuntimeSubject, String> {
    let Some(Extension(context)) = context else {
        return Err("authenticated runtime context is required".to_owned());
    };
    let organization_id = match context.organization_id.as_deref() {
        None | Some("") | Some("0") => 0,
        Some(value) => parse_non_negative_context_i64(value, "organization_id")?,
    };
    Ok(NumericRuntimeSubject {
        tenant_id: parse_positive_context_i64(&context.tenant_id, "tenant_id")?,
        organization_id,
        user_id: parse_positive_context_i64(&context.user_id, "user_id")?,
    })
}

fn parse_non_negative_context_i64(value: &str, field_name: &'static str) -> Result<i64, String> {
    let parsed = parse_int(value).ok_or_else(|| {
        format!("authenticated runtime context {field_name} must be a non-negative integer")
    })?;
    if parsed < 0 {
        return Err(format!(
            "authenticated runtime context {field_name} must be a non-negative integer"
        ));
    }
    Ok(parsed)
}

fn parse_positive_context_i64(value: &str, field_name: &'static str) -> Result<i64, String> {
    let parsed = parse_int(value).ok_or_else(|| {
        format!("authenticated runtime context {field_name} must be a positive integer")
    })?;
    if parsed <= 0 {
        return Err(format!(
            "authenticated runtime context {field_name} must be a positive integer"
        ));
    }
    Ok(parsed)
}
