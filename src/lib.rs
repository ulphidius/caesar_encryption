use std::collections::HashMap;

#[derive(Clone)]
pub struct CesarConfig {
    key_value: Option<u8>,
    group_size: Option<usize>,
    number_of_posibility: Option<u64>,
    index_digit_number: Option<u8>,
    start_index: Option<u8>,
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
            start_index: None,
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
            start_index: Some(65),
            decrypt_alphabet: Some(Box::new(decrypt_alphabet)),
            encrypt_alphabet: Some(Box::new(encrypt_alphabet))
        
        };
    }
    
    fn generate_default_alphabet() -> (HashMap<u8, char>, HashMap<char, u8>) {
        let mut decrypt_alphabet: HashMap<u8, char> = HashMap::new();
        let mut encrypt_alphabet: HashMap<char, u8> = HashMap::new();

        (65 .. 91).into_iter()
            .map(|element| element as u8)
            .for_each(|char_value| {
                decrypt_alphabet.insert(char_value, char_value as char);
                encrypt_alphabet.insert(char_value as char, char_value);
            });

        return (decrypt_alphabet, encrypt_alphabet);
    }

    fn is_set(&self) -> bool {
        if self.key_value.is_none() { return false };
        if self.group_size.is_none() { return false };
        if self.number_of_posibility.is_none() { return false };
        if self.index_digit_number.is_none() { return false };
        if self.start_index.is_none() { return false };
        if self.decrypt_alphabet.is_none() { return false };
        if self.encrypt_alphabet.is_none() { return false };

        return true;
    }

    pub fn key_value(mut self, key_value: u8) -> Self {
        self.key_value = Some(key_value);
        return self;
    }

    pub fn group_size(mut self, group_size: usize) -> Self {
        self.group_size = Some(group_size);
        return self;
    }

    pub fn number_of_posibility(mut self, number_of_posibility: u64) -> Self {
        self.number_of_posibility = Some(number_of_posibility);
        return self;
    }

    pub fn index_digit_number(mut self, index_digit_number: u8) -> Self {
        self.index_digit_number = Some(index_digit_number);
        return self;
    }
    
    pub fn start_index(mut self, start_index: u8) -> Self {
        self.start_index = Some(start_index);
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

    pub fn encrypt_word(&self, word: &str) -> Result<String, &'static str> {
        if ! self.is_set() {
            return Err("The config MUST be set");
        }

        if word.is_empty() {
            return Ok("".to_string());
        }

        let add_key_value = |characters: Vec<u64>| -> Vec<u64> {
            return characters.iter()
                .map(|value| (value + self.key_value.unwrap() as u64) % (self.number_of_posibility.unwrap() + self.start_index.unwrap() as u64))
                .collect::<Vec<u64>>();
        };

        let word_string_number = match self.word_to_byte_characters(word) {
            Ok(content) => content,
            Err(message) => return Err(message)
        };

        let string_end_completed = self.add_missing_character(word_string_number);

        let word_string_number_vector = match self.characters_to_string_vector(&string_end_completed) {
            Ok(content) => content,
            Err(message) => return Err(message) 
        };

        let encrypted_characters = match self.string_vector_to_numeric_vector(word_string_number_vector) {
            Ok(content) => add_key_value(content),
            Err(message) => return Err(message),
        };

        return Ok(self.numbers_to_string_number(encrypted_characters).into_iter()
            .map(|character_string| character_string.clone() )
            .collect::<String>()
        );

    }

    fn word_to_byte_characters(&self, word: &str) -> Result<String, &'static str> {
        let mut characters: String = String::new();
        
        for character in word.chars() {
            if ! self.encrypt_alphabet.as_ref().unwrap().contains_key(&character) {
                return Err("A character of the message doesn't exist in the current alphabet");
            }

            characters = [
                characters, 
                self.encrypt_alphabet.as_ref().unwrap().get(&character).unwrap().to_string()
            ].concat();
        }

        return Ok(characters);
    }

    fn characters_to_string_vector(&self, characters: &str) -> Result<Vec<String>, &'static str>{
        return match self.group_size {
            None => Err("Error the size of characters group undefined"),
            Some(value) if value <= 0 => Err("Error the size of characters must be upper than 0"),
            Some(_) =>  Ok(characters.chars()
                .collect::<Vec<char>>()
                .chunks(self.group_size.unwrap() * self.index_digit_number.unwrap() as usize)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()),
        };
    }

    fn string_vector_to_numeric_vector(&self, splited_word: Vec<String>) -> Result<Vec<u64>, &'static str>{
        return Ok(splited_word.iter()
            .map(|element| element.parse::<u64>().expect("The character MUST be a numeric value"))
            .map(|element| element - self.index_digit_number.unwrap() as u64)
            .collect::<Vec<u64>>());
    }

    fn numbers_to_string_number(&self, digit_character: Vec<u64>) -> Vec<String> {
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

    fn add_missing_character(&self, string_to_completed: String) -> String {
        if string_to_completed.len() % (self.group_size.unwrap() * self.index_digit_number.unwrap() as usize) == 0 {
            return string_to_completed; 
        }

        return self.add_missing_character([string_to_completed, "0".to_string()].concat());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_word_test() {
        let conf = CesarConfig::default();
        let sample = "ABC";
        let expected = String::from("676869");

        assert_eq!(conf.encrypt_word(sample).unwrap(), expected);
    }

    #[test]
    fn encrypt_word_empty_input() {
        let conf = CesarConfig::default();
        let sample = "";

        assert_eq!(conf.encrypt_word(sample).unwrap(), "");
    }

    #[test]
    fn encrypt_word_group_2_test() {
        let conf = CesarConfig::default().group_size(2).number_of_posibility(2526);
        let sample = "ABC";

        assert_eq!(conf.encrypt_word(sample).unwrap(), "15141648");
    }

    #[test]
    fn word_to_byte_characters_test() {
        let conf = CesarConfig::default();
        let sample = "ABC";
        let expected = String::from("656667");

        assert_eq!(expected, conf.word_to_byte_characters(sample).unwrap()); 
    }

    #[test]
    #[should_panic]
    fn word_to_byte_characters_lowercase() {
        let conf = CesarConfig::default();
        let sample = "aBc";
        let expected = String::from("656667");

        assert_eq!(expected, conf.word_to_byte_characters(sample).unwrap()); 
    }

    
    #[test]
    #[should_panic]
    fn word_to_byte_characters_does_not_exist() {
        let conf = CesarConfig::default();
        let sample = "&é(";

        conf.word_to_byte_characters(sample).unwrap(); 
    }

    #[test]
    fn characters_to_string_vector_test() {
        let conf = CesarConfig::default();
        let sample = "656667";
        let expected = vec![
            "65",
            "66",
            "67"
        ];

        assert_eq!(expected, conf.characters_to_string_vector(sample).unwrap());
    }

    #[test]
    #[should_panic(expected = "Error the size of characters must be upper than 0")]
    fn characters_to_string_vector_bad_group_size() {
        let mut conf = CesarConfig::default();
        conf.group_size = Some(0);
        let sample = "656667";

        conf.characters_to_string_vector(sample).unwrap();
    }


    #[test]
    #[should_panic(expected = "Error the size of characters group undefined")]
    fn characters_to_string_vector_unitialise_groupe_number() {
        let conf = CesarConfig::new();
        let sample = "656667";

        conf.characters_to_string_vector(sample).unwrap();
    }

    #[test]
    fn string_vector_to_numeric_vector_test() {
        let conf = CesarConfig::default();
        let sample = vec![
            "65".to_string(),
            "66".to_string(),
            "67".to_string()
        ];
        let expected = vec![
            65,
            66,
            67
        ];

        assert_eq!(expected, conf.string_vector_to_numeric_vector(sample).unwrap());
    }

    #[test]
    #[should_panic(expected = "The character MUST be a numeric value")]
    fn string_vector_to_numeric_vector_not_numbers() {
        let conf = CesarConfig::default();
        let sample = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string()
        ];

        conf.string_vector_to_numeric_vector(sample).unwrap();
    }

    #[test]
    fn number_to_string_number_test() {
        let conf = CesarConfig::default();
        let sample = vec![
            65,
            66,
            67
        ];
        let expected = vec![
            "65",
            "66",
            "67"
        ];

        assert_eq!(expected, conf.numbers_to_string_number(sample));
    }

    #[test]
    fn number_to_string_number_smaller_than_group_size() {
        let conf = CesarConfig::default().group_size(3);
        let sample = vec![
            5,
            56,
            67
        ];
        let expected = vec![
            "005",
            "056",
            "067"
        ];

        assert_eq!(expected, conf.numbers_to_string_number(sample));
    }

    #[test]
    fn add_missing_character_test() {
        let conf = CesarConfig::default().index_digit_number(2).group_size(3);
        let sample = "65".to_string();
        let expected = "650000";

        assert_eq!(conf.add_missing_character(sample), expected);
    }

    #[test]
    fn add_missing_character_group_by_1() {
        let conf = CesarConfig::default().index_digit_number(2).group_size(1);
        let sample = "65".to_string();
        let expected = "65";

        assert_eq!(conf.add_missing_character(sample), expected);
    }

}
