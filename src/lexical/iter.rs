use super::LexicalIter;

impl<'a> Iterator for LexicalIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.preview;
        self.preview = self.iter.next().or(Some('\0'))?;
        return Some(r);
    }
}
impl<'a> LexicalIter<'a> {
    pub fn new(data: &'a str) -> LexicalIter<'a> {
        let mut iter = LexicalIter {
            preview: '\0',
            data: data.to_string(),
            iter: data.chars(),
        };
        iter.next();
        return iter;
    }

    pub fn get_preview(&self) -> Option<char> {
        if self.preview == '\0' {
            None
        } else {
            Some(self.preview)
        }
    }
}
