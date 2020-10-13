use std::collections::HashMap;

#[derive(Clone)]
pub struct CesarConfig {
    key_value: Option<i32>,
    group_size: Option<usize>,
    number_of_posibilities: Option<u64>,
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
            number_of_posibilities: None,
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
            number_of_posibilities: Some(26),
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
        if self.number_of_posibilities.is_none() { return false };
        if self.index_digit_number.is_none() { return false };
        if self.start_index.is_none() { return false };
        if self.decrypt_alphabet.is_none() { return false };
        if self.encrypt_alphabet.is_none() { return false };

        return true;
    }

    fn is_valid(&self) -> Result<(), &'static str> {
        if self.key_value.unwrap() == 0 { return Err("The key value MUST be greater than 0") };
        if self.group_size.unwrap() == 0 { return Err("The groupe size MUST be greater than 0") };
        if self.index_digit_number.unwrap() == 0 { return Err("The number of digit MUST be greater than 0") };

        return Ok(());
    }

    pub fn key_value(mut self, key_value: i32) -> Self {
        self.key_value = Some(key_value);
        return self;
    }

    pub fn group_size(mut self, group_size: usize) -> Self {
        self.group_size = Some(group_size);
        return self;
    }

    pub fn number_of_posibilities(mut self, number_of_posibilities: u64) -> Self {
        self.number_of_posibilities = Some(number_of_posibilities);
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

        self.is_valid()?;

        if word.is_empty() {
            return Ok("".to_string());
        }

        let number_of_posibities_by_group = self.number_of_posibilities_by_group_size();

        let mut byte_vector_of_word = self.word_to_byte_vector(word)?;
        byte_vector_of_word = self.remove_index_value(byte_vector_of_word);
        println!("removed index {:?}", byte_vector_of_word);

        let splited_into_bytes = self.groups_bytes_character(byte_vector_of_word.into_iter()
            .map(|value| value as u64)
            .collect());

        println!("{:?}", splited_into_bytes);

        let added_key = self.add_key_value(
            splited_into_bytes.clone()
        );
        println!("added key {:?}", added_key);

        let number_to_string = self.numbers_to_string_number(
            added_key      
        );
        println!("{:?}", number_to_string);

        return Ok(self.output_formater(
            number_to_string
        ));
    }


    fn number_of_posibilities_by_group_size(&self) -> u64 {
        let mut result = String::new();
    
        for _ in 0..self.group_size.unwrap()  {
            result = [result, (self.group_size.unwrap() - 1).to_string()].concat();
        }
    
        return result.parse::<u64>().unwrap() + 1;
    }

    fn word_to_byte_vector(&self, word: &str) -> Result<Vec<u8>, &'static str> {
        let mut bytes_vector: Vec<u8> = vec![];
        
        for character in word.chars() {
            if ! self.encrypt_alphabet.as_ref().unwrap().contains_key(&character) {
                return Err("A character of the message doesn't exist in the current alphabet");
            }

            bytes_vector.push(*self.encrypt_alphabet.as_ref().unwrap()
                .get(&character).unwrap());

        }

        return Ok(bytes_vector);
    }

    fn remove_index_value(&self, byte_characters: Vec<u8>) -> Vec<u8> {
        return byte_characters.into_iter()
            .map(|character| character - self.start_index.unwrap())
            .collect();
    }

    fn groups_bytes_character(&self, byte_characters: Vec<u64>) -> Vec<u64> {
        if byte_characters.is_empty() {
            return vec![];
        }

        if byte_characters.len() == 1 {
            return byte_characters;
        }

        return vec![evaluate_grouped_value(byte_characters[..self.group_size.unwrap()].to_vec())]
            .into_iter()
            .chain(
                self.groups_bytes_character(byte_characters[self.group_size.unwrap()..].to_vec())
                .into_iter()
            )
            .collect();
    }

    fn add_key_value(&self, list_of_values: Vec<u64>) -> Vec<u64> {
        if self.key_value.unwrap() < 0 {
            return list_of_values.into_iter()
                .map(|value| {
                    if (value as i128) < self.key_value.unwrap() as i128 {
                        return  (value as i128 + self.number_of_posibilities.unwrap() as i128 + self.key_value.unwrap() as i128) as u64;
                    }

                    return ((value as i128 + self.key_value.unwrap() as i128) % self.number_of_posibilities.unwrap() as i128) as u64;
                })
                .collect();
        }

        return list_of_values.into_iter()
            .into_iter()
            .map(|value| ((value as i128 + self.key_value.unwrap() as i128) % self.number_of_posibilities.unwrap() as i128) as u64)
            .collect();
    }

    fn numbers_to_string_number(&self, digit_characters: Vec<u64>) -> Vec<String> {
        println!("before change: {:?}", digit_characters);
        return digit_characters.into_iter()
            .map(|value| value.to_string())
            .collect();
    }

    fn output_formater(&self, list_of_values: Vec<String>) -> String {
        // if self.group_size.unwrap() == 1 {
        //     return list_of_values.into_iter()
        //         .map(|value| self.decrypt_alphabet.unwrap().as_ref().get(value.parse::<u8>()).unwrap())
        //         .collect();
        // }

        return list_of_values.into_iter()
            .map(|element| [element, "-".to_string()].concat())
            .collect();
    }

    fn add_missing_character(&self, string_to_completed: String) -> String {
        if string_to_completed.len() % (self.group_size.unwrap() * self.index_digit_number.unwrap() as usize) == 0 {
            return string_to_completed; 
        }

        return self.add_missing_character([string_to_completed, "0".to_string()].concat());
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
        let value_to_remove: u64 = self.start_index.unwrap() as u64 * (10 as u64).pow(self.group_size.unwrap() as u32);

        return Ok(splited_word.iter()
            .map(|element| element.parse::<u64>().expect("The character MUST be a numeric value"))
            .map(|element| element - value_to_remove)
            .collect::<Vec<u64>>());
    }

}

fn evaluate_grouped_value(elements_to_group: Vec<u64>) -> u64 {
    let mut result = elements_to_group[0];

    for element in elements_to_group[1..].into_iter() {
        result = result * 100 + element; 
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_word_test() {
        let conf = CesarConfig::default();
        let sample = "ABC";
        let expected = String::from("234");

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
        let conf = CesarConfig::default().group_size(2).number_of_posibilities(2526);
        let sample = "ABC";

        assert_eq!(conf.encrypt_word(sample).unwrap(), "15141648");
    }

    #[test]
    fn word_to_byte_characters_test() {
        let conf = CesarConfig::default();
        let sample = "ABC";
        let expected: Vec<u8> = vec![65, 66, 67];

        assert_eq!(expected, conf.word_to_byte_vector(sample).unwrap()); 
    }

    #[test]
    #[should_panic]
    fn word_to_byte_characters_lowercase() {
        let conf = CesarConfig::default();
        let sample = "aBc";

        conf.word_to_byte_vector(sample).unwrap(); 
    }

    
    #[test]
    #[should_panic]
    fn word_to_byte_characters_does_not_exist() {
        let conf = CesarConfig::default();
        let sample = "&é(";

        conf.word_to_byte_vector(sample).unwrap(); 
    }

    #[test]
    fn remove_index_value_test() {
        let conf = CesarConfig::default().start_index(65);
        let sample: Vec<u8> = vec![65, 66, 67];
        let expected: Vec<u8> = vec![0, 1, 2];

        assert_eq!(expected, conf.remove_index_value(sample)); 
    }

    #[test]
    fn remove_index_value_empty_input() {
        let conf = CesarConfig::default().start_index(65);
        let sample: Vec<u8> = vec![];
        let expected: Vec<u8> = vec![];

        assert_eq!(expected, conf.remove_index_value(sample));
    }

    #[test]
    fn remove_index_value_too_low_values() {
        let conf = CesarConfig::default().start_index(65);
        let sample: Vec<u8> = vec![0, 1, 2];
        let expected: Vec<u8> = vec![];

        assert_eq!(expected, conf.remove_index_value(sample));
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
            0,
            1,
            2
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
