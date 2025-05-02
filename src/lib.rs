use reqwest::blocking;
use std::{path::Path, time::SystemTime};

/// Simple XOR encryption key used for basic obfuscation
const ENCRYPTION_KEY: u8 = 42;
pub fn encrypt_api_key(input: &str) -> String {
    // XOR each byte with the encryption key
    let encrypted_bytes: Vec<u8> = input.bytes().map(|byte| byte ^ ENCRYPTION_KEY).collect();

    // Convert to base64 for storage/transmission
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, encrypted_bytes)
}

/// Decrypts a base64-encoded XOR-encrypted string
///
/// # Arguments
/// * `encrypted_input` - The base64-encoded encrypted string
///
/// # Returns
/// The decrypted original string, or an empty string if decryption fails
pub fn decrypt_api_key(encrypted_input: &str) -> String {
    // Decode from base64
    if let Ok(decoded) =
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted_input)
    {
        // XOR each byte with the same key to get the original value
        let decrypted_bytes: Vec<u8> = decoded.iter().map(|byte| byte ^ ENCRYPTION_KEY).collect();

        // Convert back to a string
        if let Ok(result) = String::from_utf8(decrypted_bytes) {
            return result;
        }
    }

    // Return empty string if decryption fails
    String::new()
}

/// Reads the encrypted API key from a file
pub fn read_encrypted_api_key(filepath: &str) -> Result<String, std::io::Error> {
    let path = Path::new(filepath);
    if path.exists() {
        std::fs::read_to_string(filepath)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", filepath),
        ))
    }
}
/// Emits a POST request to renew the API key
pub fn renew_api_key(url: &str, current_key: &str) -> Result<(), ()> {
    println!("{}", current_key);
    let client: blocking::Client = blocking::Client::new();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let name = format!("ALBERTINE-{}", now);
    let params = [("name", name)];
    "name=Albertine";
    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", "current_key"))
        .form(&params)
        .send()
        .expect("Impossible d'obtenir la réponse à la requête de renouvellement de clé");

    println!("Renewal response: {:?}", res);

    if res.status().is_success() {
        let new_key = res.json::<String>().unwrap();
        println!("New API key: {}", new_key);
        Ok(())
    } else {
        Err(())
    }
}
