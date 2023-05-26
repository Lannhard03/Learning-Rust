pub use crate::spellchecker::SpellChecker;
pub use crate::spellchecker::Text;
use std::fs;
use std::io::Write;

pub struct SpellWarning<'a> {
    word: &'a str,
    corrections: Vec<String>,
    line: usize,
}

pub struct Report<'a> {
    spell_warnings: Vec<SpellWarning<'a>>,
    report_name: String,
}

pub struct WordFrequencyData {
    frequency_data: Vec<String>,
}

pub struct Config {
    text_paths: Vec<String>,
    data_path: String,
    number_of_correction: usize,
}

impl Config {
    pub fn new(text_paths: Vec<String>, data_path: String, number_of_correction: usize) -> Self {
        Self {
            text_paths,
            data_path,
            number_of_correction,
        }
    }
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let data_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a data path"),
        };
        let text_paths: Vec<String> = args.collect();
        match text_paths.len() {
            0 => return Err("Didnt' get any text paths"),
            _ => (),
        };

        Ok(Config {
            text_paths,
            data_path,
            number_of_correction: 3,
        })
    }
}

pub fn run(config: &Config) -> Result<(), &'static str> {
    let data = match WordFrequencyData::load_data(&config.data_path) {
        Ok(loaded_data) => loaded_data,
        Err(_) => return Err("Problem loading data"),
    };
    let texts: Vec<Text> = match config
        .text_paths
        .iter()
        .filter_map(|text_path| Some(Text::load_text_from_path(text_path)))
        .collect()
    {
        Ok(loaded_texts) => loaded_texts,
        Err(_) => return Err("Problem loading texts"),
    };

    let spell_checker = SpellChecker::new(texts, config.number_of_correction, data);
    let reports = spell_checker.spell_check_all_texts();
    for report in reports {
        if let Err(_) = report.create_report_file() {
            return Err("Problem creating report with name");
        }
    }

    Ok(())
}

impl<'a> SpellWarning<'a> {
    pub fn new(word: &'a String, corrections: Vec<String>, line: usize) -> Self {
        Self {
            word,
            corrections,
            line,
        }
    }
}

impl<'a> ToString for SpellWarning<'a> {
    fn to_string(&self) -> String {
        let mut corrections_string = String::from("");
        for correction in &self.corrections {
            corrections_string += correction;
            corrections_string.push_str(", ")
        }
        format!("[{}] {}: {}", self.line, self.word, corrections_string)
    }
}

impl WordFrequencyData {
    fn new(frequency_data: Vec<String>) -> Self {
        Self { frequency_data }
    }

    pub fn load_data(data_path: &str) -> Result<WordFrequencyData, std::io::Error> {
        let contents = fs::read_to_string(data_path)?;
        let mut data = Vec::new();
        for line in contents.split("\n") {
            let data_string = String::from(line.split("\t").next().unwrap());
            data.push(data_string);
        }
        let word_data = WordFrequencyData::new(data);
        Ok(word_data)
    }

    pub fn word_in_data(&self, word: &String) -> bool {
        self.frequency_data.contains(word)
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.frequency_data
    }
}

impl<'a> Report<'a> {
    pub fn create_report_file(&self) -> Result<(), std::io::Error> {
        println!("Creating report in file: {}", self.report_name);
        let mut report_file = fs::File::create(format!("reports/{}", self.report_name))?;
        let header = format!("Spell check for text in {}", self.report_name);
        report_file.write_all(&header[..].as_bytes())?;
        report_file.write_all(b"\n")?;
        for spell_error in &self.spell_warnings {
            report_file.write_all(&spell_error.to_string().as_bytes())?;
            report_file.write_all(b"\n")?;
        }
        Ok(())
    }

    pub fn new(spell_warnings: Vec<SpellWarning<'a>>, report_name: String) -> Self {
        Report {
            spell_warnings,
            report_name,
        }
    }

    pub fn report_name(&self) -> &String {
        &self.report_name
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spell_warning() {
        let binding = String::from("hen");
        let warning = SpellWarning::new(
            &binding,
            vec![
                String::from("hej"),
                String::from("hon"),
                String::from("han"),
            ],
            1,
        );
        assert_eq!(warning.to_string(), "[1] hen: hej, hon, han, ");
    }
    #[test]
    fn test_load_data() {
        let passed;
        match WordFrequencyData::load_data("data/attasidor_stats.tsv") {
            Err(e) => {
                passed = false;
                println!("{e}");
            }
            Ok(data) => passed = data.frequency_data[0] == "i",
        }
        assert!(passed)
    }

    #[test]
    fn test_word_in_data() {
        let data = WordFrequencyData::load_data("data/attasidor_stats.tsv").unwrap();
        let mut passed = data.word_in_data(&String::from("hej"));
        passed &= !data.word_in_data(&String::from("hjsdfkks"));
        assert!(passed)
    }

    #[test]
    fn test_read_to_file() {
        let binding = String::from("hen");
        let binding = SpellWarning::new(
            &binding,
            vec![
                String::from("hej"),
                String::from("hon"),
                String::from("han"),
            ],
            1,
        );
        let report = Report::new(vec![binding], String::from("jena"));
        let result = report.create_report_file();
        match result {
            Err(e) => {
                print!("{e}");
                assert!(false);
            }
            Ok(()) => assert!(true),
        }
    }
}
