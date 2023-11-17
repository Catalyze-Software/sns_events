use ic_scalable_misc::{
    enums::{api_error_type::ApiError, validation_type::ValidationType},
    helpers::validation_helper::Validator,
    models::validation_models::ValidateField,
};

use shared::event_models::{PostEvent, UpdateEvent};

pub fn validate_post_event(post_event: PostEvent) -> Result<(), ApiError> {
    let validator_fields = vec![
        ValidateField(
            ValidationType::StringLength(post_event.name, 3, 64),
            "name".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(post_event.description, 0, 500),
            "description".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(post_event.website, 0, 200),
            "website".to_string(),
        ),
        ValidateField(
            ValidationType::Count(post_event.tags.len(), 0, 50),
            "causes".to_string(),
        ),
    ];

    Validator(validator_fields).validate()
}

pub fn validate_update_event(update_event: UpdateEvent) -> Result<(), ApiError> {
    let validator_fields = vec![
        ValidateField(
            ValidationType::StringLength(update_event.name, 3, 64),
            "name".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(update_event.description, 0, 500),
            "description".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(update_event.website, 0, 200),
            "website".to_string(),
        ),
        ValidateField(
            ValidationType::Count(update_event.tags.len(), 0, 50),
            "causes".to_string(),
        ),
    ];

    Validator(validator_fields).validate()
}
