use std::io::{Read, BufReader, BufRead};

pub trait Preprocessor {
    fn process<'a, R: Read>(&mut self, file: &mut PreprocessIterator<'a, R>);
    fn activate<'a, R: Read>(&mut self, params: Vec<String>, file: &mut PreprocessIterator<'a, R>);
}

pub struct PreprocessIterator<'a, R: Read> {
    current_line: String,
    reader: &'a mut BufReader<R>,
    reprocess_lines: Vec<String>
}

impl<'a, R: Read> PreprocessIterator<'a, R> {
    fn new(reader: &'a mut BufReader<R>) -> PreprocessIterator<R> {
        PreprocessIterator {
            current_line: String::new(),
            reprocess_lines: vec![],
            reader
        }
    }

    pub fn current(&self) -> &String {
        &self.current_line
    }

    pub fn append_lines(&mut self, mut lines: Vec<String>) {
        lines.reverse();
        self.reprocess_lines.append(&mut lines);
    }
}

impl<'a, R: Read> Iterator for PreprocessIterator<'a, R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.reprocess_lines.pop() {
            self.current_line = line.clone();
            Some(line)
        }
        else {
            if self.reader.read_line(&mut self.current_line).expect("Error accoured while reading file (preprocessing)") > 0 {
                Some(self.current_line.clone())
            }
            else {
                None
            }
        }
    }
}