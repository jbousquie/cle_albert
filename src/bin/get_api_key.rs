use cle_albert::{decrypt_api_key, encrypt_api_key, read_encrypted_api_key, renew_api_key};
use std::env;

pub const API_URL: &str = "https://albert.api.etalab.gouv.fr/v1/tokens";
const KEY_FILENAME: &str = "albertine.key";

fn main() -> Result<(), ()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    let key_file = if args.len() > 1 {
        args[1].as_str()
    } else {
        KEY_FILENAME
    };

    let mut current_key: String;
    if let Ok(read_key) = read_encrypted_api_key(key_file) {
        current_key = read_key.clone();
        if current_key.ends_with('\n') {
            current_key.pop();
        }
    } else {
        println!("Erreur lecture du fichier {}", key_file);
        return Err(());
    }
    if current_key.is_empty() {
        println!("Erreur : API Key vide");
        return Err(());
    }
    let mut decrypted_key = decrypt_api_key(&current_key);
    if decrypted_key.is_empty() {
        println!("Erreur : API Key invalide");
        return Err(());
    }
    if decrypted_key.ends_with('\n') {
        decrypted_key.pop();
    }
    if decrypted_key.ends_with('\r') {
        decrypted_key.pop();
    }
    renew_api_key(API_URL, &decrypted_key)
}
