use crate::error::CadetHubError;
use crate::logger::info;
use crate::CadetHubResult;
use keyring::Entry;

pub fn get_or_create_admin_key(service_name: &str) -> CadetHubResult<String> {
    get_or_create_key(service_name, "admin")
}

pub fn get_or_create_key(service: &str, user: &str) -> CadetHubResult<String> {
    info!("Get or create key: service={}, user={}", service, user);
    let entry = Entry::new(service, user).map_err(CadetHubError::general_error_with_source)?;
    match entry.get_password() {
        Ok(existing_key) => Ok(existing_key),

        Err(keyring::Error::NoEntry) => {
            let key = uuid::Uuid::new_v4().to_string();
            info!("Create key: service={}, user={}", service, user);
            entry
                .set_password(&key)
                .map_err(CadetHubError::general_error_with_source)
                .map(|_| key)
        }

        Err(error) => Err(CadetHubError::general_error_with_source(error)),
    }
}

pub fn delete_key(service: &str, user: &str) -> CadetHubResult<()> {
    info!("Delete key: service={}, user={}", service, user);
    Entry::new(service, user)
        .and_then(|entry| entry.delete_credential())
        .map_err(CadetHubError::general_error_with_source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    fn clear_keyring(service: &str, user: &str) {
        let entry = Entry::new(service, user).expect("failed create entry");
        if let Err(error) = entry.delete_credential() {
            println!("{:?}", error);
        }
    }

    #[test]
    fn should_get_or_create_key() {
        // Given
        let service = "test-cadet-hub-service-1";
        let user = "test-cadet-hub-admin-1";
        clear_keyring(service, user);
        let key = get_or_create_key(service, user).expect("failed get or create key");

        // When
        let result = get_or_create_key(service, user).expect("failed get or create key");

        // Then
        assert_that!(result).is_equal_to(key);
        clear_keyring(service, user);
    }

    #[test]
    fn should_delete_key() {
        // Given
        let service = "test-cadet-hub-service-2";
        let user = "test-cadet-hub-admin-2";
        clear_keyring(service, user);
        get_or_create_key(service, user).expect("failed get or create key");

        // When
        let result = delete_key(service, user);

        // Then
        assert_that!(result).is_ok();
        clear_keyring(service, user);
    }

    #[test]
    fn should_delete_key_case_non_existent_key() {
        // Given
        let service = "test-cadet-hub-service-3";
        let user = "test-cadet-hub-admin-3";
        clear_keyring(service, user);

        // When
        let result = delete_key(service, user);

        // Then
        assert_that!(result).is_err();
    }
}
