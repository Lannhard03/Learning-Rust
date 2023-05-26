use crate::data::Report;
use crate::data::SpellWarning;
use crate::data::WordFrequencyData;
use crate::med;
use rayon::prelude::*;
use regex::Regex;
use std::{fs, path::Path};

pub struct Text {
    name: String,
    words: Vec<(String, usize)>,
}
pub struct SpellChecker {
    word_frequency_data: WordFrequencyData,
    texts: Vec<Text>,
    number_of_correction: usize,
}

impl Text {
    pub fn load_text_from_path(path: &str) -> Result<Text, std::io::Error> {
        let text_string = fs::read_to_string(path)?;

        let text_lines: Vec<&str> = text_string.split("\n").collect();
        let mut text_words: Vec<(String, usize)> = Vec::new();
        for (line_num, line_text) in text_lines.iter().enumerate() {
            let re = Regex::new(r"[\w]+").unwrap();
            let words_in_line = re.find_iter(line_text);
            for word in words_in_line {
                text_words.push((String::from(word.as_str().to_lowercase()), line_num));
            }
        }

        Ok(Text {
            words: text_words,
            name: String::from(Path::new(path).file_name().unwrap().to_str().unwrap()),
        })
    }

    pub fn get_words(&self) -> &Vec<(String, usize)> {
        &self.words
    }
}

impl<'a> SpellChecker {
    fn spell_check_text(&self, text: &'a Text) -> Vec<SpellWarning<'a>> {
        let misspellt_words = text.get_words().into_par_iter().filter(|x| {
            let word = &x.0;
            !self.word_frequency_data.word_in_data(&word)
        });
        let spell_warnings = misspellt_words
            .map(|misspelling| {
                SpellWarning::new(
                    &misspelling.0,
                    self.find_corrections(&misspelling.0),
                    misspelling.1,
                )
            })
            .collect();
        spell_warnings
    }

    fn find_corrections(&self, word: &String) -> Vec<String> {
        let filtered_data = self
            .word_frequency_data
            .get_data()
            .into_par_iter()
            //.filter(|x| word.len() + 3 > x.len() && x.len() > word.len() - 3)
            .map(|x| (x, med::levenshtien_distance(x, word)));
        let sync_data: Vec<(&String, usize)> = filtered_data.collect();
        let mut possible_corrections: Vec<(&String, usize)>;
        //Create vec with wanted number of random correction guesses,
        //then processes to find best guesses
        if sync_data.len() > self.number_of_correction {
            possible_corrections = sync_data[1..=self.number_of_correction].to_vec();
        } else {
            return sync_data.iter().map(|x| x.0.clone()).collect();
        }
        //This code is pretty bad, but to allow for more than one correction it is needed...
        let mut insert_element = false;
        let mut insert_index = 0;
        for data in sync_data {
            for (i, ele) in possible_corrections.iter().enumerate() {
                insert_element = false;
                if ele.1 > data.1 {
                    insert_index = i;
                    insert_element = true;
                    break;
                }
            }
            if insert_element {
                possible_corrections.insert(insert_index, data);
                possible_corrections.pop();
            }
        }

        possible_corrections.iter().map(|x| x.0.clone()).collect()
    }

    pub fn spell_check_all_texts(&self) -> Vec<Report> {
        let reports = self
            .texts
            .iter()
            .map(|text| Report::new(self.spell_check_text(&text), text.name.clone()))
            .collect();
        reports
    }

    pub fn new(
        texts: Vec<Text>,
        number_of_correction: usize,
        word_frequency_data: WordFrequencyData,
    ) -> Self {
        Self {
            word_frequency_data,
            texts,
            number_of_correction,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_text() {
        let text = Text::load_text_from_path("texter/kort1.txt").expect("Error while loading text");
        assert_eq!(text.words[0], (String::from("en"), 0));
    }

    #[test]
    fn test_autocorrect() {
        let data = WordFrequencyData::load_data("data/attasidor_stats.tsv").unwrap();
        let spell_checker = SpellChecker {
            word_frequency_data: data,
            texts: vec![Text {
                words: vec![(String::from("hej"), 1)],
                name: String::from("temp"),
            }],
            number_of_correction: 3,
        };
        let corrections = spell_checker.find_corrections(&String::from("hon"));
        assert_eq!(corrections[0], "hon")
    }
}
