use std::collections::HashMap;

pub struct CesarConfig {
    key_value: Option<u8>,
    group_size: Option<usize>,
    number_of_posibility: Option<u8>,
    index_digit_number: Option<u8>,
    decrypt_alphabet: Option<Box<HashMap<u8, char>>>,
    encrypt_alphabet: Option<Box<HashMap<char, u8>>>
}

impl CesarConfig {
    pub fn new() -> Self {
        return Self {
            key_value: None,
            group_size: None,
            number_of_posibility: None,
            index_digit_number: None,
            decrypt_alphabet: None,
            encrypt_alphabet: None
        };
    }

    pub fn default() -> Self {
        let (decrypt_alphabet, encrypt_alphabet) = CesarConfig::generate_default_alphabet();

        return Self {
            key_value: Some(2),
            group_size: Some(1),
            number_of_posibility: Some(26),
            index_digit_number: Some(2),
            decrypt_alphabet: Some(Box::new(decrypt_alphabet)),
            encrypt_alphabet: Some(Box::new(encrypt_alphabet))
        
        };
    }
    
    fn generate_default_alphabet() -> (HashMap<u8, char>, HashMap<char, u8>) {
        let mut decrypt_alphabet: HashMap<u8, char> = HashMap::new();
        let mut encrypt_alphabet: HashMap<char, u8> = HashMap::new();

        (65 .. 91).into_iter()
            .map(|element| element as u8)
            .enumerate()
            .for_each(|(index, char_value)| {
                decrypt_alphabet.insert(index as u8, char_value as char);
                encrypt_alphabet.insert(char_value as char, index as u8);
            });

        return (decrypt_alphabet, encrypt_alphabet);
    }

    pub fn key_value(mut self, key_value: u8) -> Self {
        self.key_value = Some(key_value);
        return self;
    }

    pub fn group_size(mut self, group_size: usize) -> Self {
        self.group_size = Some(group_size);
        return self;
    }

    pub fn number_of_posibility(mut self, number_of_posibility: u8) -> Self {
        self.number_of_posibility = Some(number_of_posibility);
        return self;
    }

    pub fn index_digit_number(mut self, index_digit_number: u8) -> Self {
        self.index_digit_number = Some(index_digit_number);
        return self;
    }

    pub fn decrypt_alphabet(mut self, decrypt_alphabet: HashMap<u8, char>) -> Self {
        self.decrypt_alphabet = Some(Box::new(decrypt_alphabet));
        return self;
    }

    pub fn encrypt_alphabet(mut self, encrypt_alphabet: HashMap<char, u8>) -> Self {
        self.encrypt_alphabet = Some(Box::new(encrypt_alphabet));
        return self;
    }

    // pub fn encrypt_character<'a>(self, characters: &'a str) -> Result<String, &'static str> {
    //     let add_key = |value: &str| -> String {
    //         return (value.parse::<u8>().unwrap() + self.key_value.unwrap() % self.number_of_posibility.unwrap()).to_string()
    //     };

    //     return match self.group_size {
    //         Some(value) if value <= 0 => Err("Error the size of characters must be upper than 0"),
    //         Some(value) if value > 0 => {
    //             let character_value = *self.encrypt_alphabet.unwrap().get().unwrap();
    //             let encrypted_characters = str_to_string_vector(characters).into_iter()
    //                 .map(|string_character| {
    //                     let character_value = *self.encrypt_alphabet.unwrap().get(&string_character).unwrap(); 
    //                     add_key()
    //                 })
    //                 .collect::<String>();
    //             return Ok(encrypted_characters);
    //         },
    //         Some(_) => panic!("The groupe size is Quantum !!!!!!!!!!!!!!!!!!!!!!!"),
    //         None => Err("Error the size of characters group undefined")
    //     }
    // }

    fn word_to_byte_characters(self, word: &str) -> String {
        return word.chars()
            .map(|character| self.encrypt_alphabet.as_ref().unwrap().get(&character))
            .map(|digit_character| digit_character.unwrap().to_string())
            .collect::<String>();
    }

    fn number_to_string_number(self, digit_character: Vec<u8>) -> Vec<String> {
        return digit_character.iter()
            .map(|number| number.to_string())
            .map(|mut string_number| {
                if string_number.len() < self.group_size.unwrap() {
                    while string_number.len() < self.group_size.unwrap() {
                        string_number = ["0", &string_number].concat();
                    }

                    return string_number;
                }

                return string_number;
            })
            .collect();
    }

    fn characters_to_string_vector(self, characters: &str) -> Result<Vec<String>, &'static str>{
        let chunks_size = self.group_size.unwrap() * self.index_digit_number.unwrap() as usize;

        return match self.group_size {
            Some(value) if value <= 0 => Err("Error the size of characters must be upper than 0"),
            Some(_) => Ok(characters.chars()
                .collect::<Vec<char>>()
                .chunks(chunks_size)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()),
            None => Err("Error the size of characters group undefined"),
        };
    }

    fn string_vector_to_u8_vector(self, splited_word: Vec<String>) -> Result<Vec<u8>, &'static str>{
        return Ok(splited_word.iter()
            .map(|element| element.parse::<u8>().unwrap())
            .collect::<Vec<u8>>());
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
