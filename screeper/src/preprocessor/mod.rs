use std::io::{Read, BufReader, BufRead};

/// An interface for preprocessor componants
/// 
/// # How to implement
/// ```
/// use screeper::preprocessor::Preprocessor;
/// 
/// struct PrefixPreprocessor {
///     prefix: String,
///     skip: bool
/// }
/// 
/// impl Preprocessor for PrefixPreprocessor {
///     fn process<'a, R: Read>(&mut self, file: &mut PreprocessIterator<'a, R>) -> bool {
///         if !skip && !self.prefix.is_empty() && !file.current().starts_with(&self.prefix) {
///             file.append_lines(vec![format!("{} {}", self.prefix, file.current)]);
///             self.skip = true // reappend line and force reprocessing
///         }
///         else {
///             self.skip = false
///         }
///     }
/// 
///     fn activate<'a, R: Read>(&mut self, params: Vec<String>, file: &mut PreprocessIterator<'a, R>) {
///         if params.is_empty() {
///             self.prefix.clear();
///         }
///         else {
///             self.prefix = params[0];
///         }
///     }
/// }
/// ```
pub trait Preprocessor {
    /// Processes the current line the file is at. 
    /// Returns `true` if all preceding preprocessors should skip the current line.
    fn process<'a, R: Read>(&mut self, file: &mut PreprocessIterator<'a, R>) -> bool;

    /// Runs the preprocessor header associated with the preprocessor.
    /// 
    /// The `params` parameter contains all the parameters that were given to the activation.
    /// For example, if this was the activation line:
    /// `#<name> hello 1 "multi word"`
    /// params will contain: `["hello", "1", "multi word"]`
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

