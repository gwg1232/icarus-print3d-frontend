mod sign_in;
mod sign_out;
mod sign_up;

pub use sign_in::post_forms_sign_in;
pub use sign_out::post_forms_sign_out;
pub use sign_up::post_forms_sign_up;

use std::collections::HashMap;

fn parse_validation_errors(
    validation_errors: &validator::ValidationErrors,
) -> HashMap<String, String> {
    validation_errors
        .field_errors()
        .iter()
        .filter_map(|(field, errs)| {
            errs.first()
                .and_then(|e| e.message.as_ref())
                .map(|msg| (field.to_string(), msg.to_string()))
        })
        .collect()
}
