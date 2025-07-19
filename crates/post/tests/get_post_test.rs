#[actix_web::test]
async fn user_can_get_post_with_valid_post_id() {
    // Given user passing a post ID when request
    // When the post ID is valid and existed
    // Then should return a post object

    unimplemented!()
}

#[actix_web::test]
async fn user_will_receive_post_not_found_error_when_post_does_not_exist() {
    // Given user passing a post ID when request
    // When the post ID is invalid or does not exist
    // Then should return a post not found error

    unimplemented!()
}

#[actix_web::test]
async fn user_will_receive_invalid_input_error_when_post_id_is_empty() {
    // Given user passing an empty post ID when request
    // When the post ID is empty
    // Then should return an invalid input error

    unimplemented!()
}

#[actix_web::test]
async fn user_will_receive_invalid_input_error_when_post_id_is_non_numeric() {
    // Given user passing a non-numeric post ID when request
    // When the post ID is non-numeric
    // Then should return an invalid input error

    unimplemented!()
}

#[actix_web::test]
async fn user_will_receive_internal_service_error_when_database_connection_fails() {
    // Given user passing a post ID when request
    // When the database connection fails
    // Then should return an internal service error

    unimplemented!()
}