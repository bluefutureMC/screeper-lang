use std::io::BufRead;

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
///     fn process(&mut self, file: &mut PreprocessIterator) -> bool {
///         if !skip && !self.prefix.is_empty() && !file.current().starts_with(&self.prefix) {
///             file.reprocess_lines(vec![format!("{} {}", self.prefix, file.current)]);
///             self.skip = true // reappend line and force reprocessing
///         }
///         else {
///             self.skip = false
///         }
///     }
/// 
///     fn activate(&mut self, params: Vec<String>, file: &mut PreprocessIterator) {
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
    fn process(&mut self, file: &mut PreprocessIterator) -> bool;

    /// Runs the preprocessor header associated with the preprocessor.
    /// 
    /// The `params` parameter contains all the parameters that were given to the activation.
    /// For example, if this was the activation line:
    /// `#<name> hello 1 "multi word"`
    /// params will contain: `["hello", "1", "multi word"]`
    fn activate(&mut self, params: Vec<String>, file: &mut PreprocessIterator);
}

/// An iterator used by preprocessors to iterate through lines of a file and insert new lines at the top.
/// 
/// # Examples
/// ```
/// use screeper::preprocessor::PreprocessIterator;
/// use std::fs::File;
/// use std::io::BufReader;
/// 
/// /* test.txt
/// kevin
/// brian
/// joe
/// millie
/// bobbie
/// joe
///  */
/// 
/// let file = File::open("test.txt").unwrap();
/// let mut iter = PreprocessIterator::new(Box::new(BufReader::new(file)));
/// 
/// for line in iter {
///     println!("{}", line);
///     if line.starts_with("joe") { 
///         iter.reprocess_lines(vec!["who is joe?", "JOE MAMA!"]);
///     }
/// }
/// 
/// /* Expected output
/// kevin
/// brian
/// joe
/// who is joe?
/// JOE MAMA!
/// millie
/// bobbie
/// joe
/// who is joe?
/// JOE MAMA!
///  */
/// ```
pub struct PreprocessIterator {
    current_line: String,
    reader: Box<dyn BufRead>,
    reprocess_lines: Vec<String>
}

impl PreprocessIterator {
    pub fn new(reader: Box<dyn BufRead>) -> PreprocessIterator {
        PreprocessIterator {
            current_line: String::new(),
            reprocess_lines: vec![],
            reader
        }
    }

    /// Retrieves the current processed line without advancing the iterator
    pub fn current(&self) -> &String {
        &self.current_line
    }

    /// Adds new lines the iterator will iterate through before moving on to the next line
    pub fn reprocess_lines(&mut self, mut lines: Vec<String>) {
        lines.reverse();
        self.reprocess_lines.append(&mut lines);
    }
}

impl Iterator for PreprocessIterator {
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

