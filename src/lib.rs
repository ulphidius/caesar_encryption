use std::collections::HashMap;

pub struct CesarConfig {
    key_value: Option<u8>,
    group_size: Option<usize>,
    number_of_posibility: Option<u8>,
    index_digit_number: Option<u8>,
    encrypt_alphabet: Option<Box<HashMap<u8, char>>>,
    decrypt_alphabet: Option<Box<HashMap<char, u8>>>
}

impl CesarConfig {
    pub fn new() -> Self {
        return Self {
            key_value: None,
            group_size: None,
            number_of_posibility: None,
            index_digit_number: None,
            encrypt_alphabet: None,
            decrypt_alphabet: None
        };
    }

    pub fn default() -> Self {
        let (encrypt_alphabet, decrypt_alphabet) = CesarConfig::generate_default_alphabet();

        return Self {
            key_value: Some(2),
            group_size: Some(1),
            number_of_posibility: Some(26),
            index_digit_number: Some(2),
            encrypt_alphabet: Some(Box::new(encrypt_alphabet)),
            decrypt_alphabet: Some(Box::new(decrypt_alphabet))
        
        };
    }
    
    fn generate_default_alphabet() -> (HashMap<u8, char>, HashMap<char, u8>) {
        let mut encrypt_alphabet: HashMap<u8, char> = HashMap::new();
        let mut decrypt_alphabet: HashMap<char, u8> = HashMap::new();

        (65 .. 91).into_iter()
            .map(|element| element as u8)
            .enumerate()
            .for_each(|(index, char_value)| {
                encrypt_alphabet.insert(index as u8, char_value as char);
                decrypt_alphabet.insert(char_value as char, index as u8);
            });

        return (encrypt_alphabet, decrypt_alphabet);
    }

    pub fn key_value(mut self, key_value: u8) -> Self {
        self.key_value = Some(key_value);
        return self;
    }

    pub fn group_size(mut self, group_size: usize) -> Self {
        self.group_size = Some(group_size);
        return self;
    }

    pub fn index_digit_number(mut self, index_digit_number: u8) -> Self {
        self.index_digit_number = Some(index_digit_number);
        return self;
    }

    pub fn encrypt_alphabet(mut self, encrypt_alphabet: HashMap<u8, char>) -> Self {
        self.encrypt_alphabet = Some(Box::new(encrypt_alphabet));
        return self;
    }

    pub fn decrypt_alphabet(mut self, decrypt_alphabet: HashMap<char, u8>) -> Self {
        self.decrypt_alphabet = Some(Box::new(decrypt_alphabet));
        return self;
    }

    pub fn encrypt_character<'a>(self, characters: &'a str) -> Result<String, &'static str> {
        let add_key = |value: &str| -> String {
            return (value.parse::<u8>().unwrap() + self.key_value.unwrap() % self.number_of_posibility.unwrap()).to_string()
        };

        let str_to_string_vector = |str_to_convert: &str| -> Vec<String> {
            return str_to_convert.chars()
                .collect::<Vec<char>>()
                .chunks(self.group_size.unwrap() * self.index_digit_number.unwrap() as usize)
                .map(|chunked_list| chunked_list.iter().collect::<String>())
                .collect::<Vec<String>>();    
        };

        return match self.group_size {
            Some(value) if value <= 0 => Err("Error the size of characters must be upper than 0"),
            Some(value) if value > 0 => {
                let encrypted_characters = str_to_string_vector(characters).into_iter()
                    .map(|string_character| add_key(&string_character))
                    .collect::<String>();
                return Ok(encrypted_characters);
            },
            Some(_) => panic!("The groupe size is Quantum !!!!!!!!!!!!!!!!!!!!!!!"),
            None => Err("Error the size of characters group undefined")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
}
