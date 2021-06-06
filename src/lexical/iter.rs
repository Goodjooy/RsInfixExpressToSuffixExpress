use super::{Lexical, LexicalData, LexicalDataIter, LexicalIter};

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

impl Iterator for LexicalDataIter {
    type Item = LexicalData;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.data.get(self.index)?;
        self.index += 1;
        Some(v.clone())
    }
}
impl LexicalDataIter {
    pub fn preview(&self) -> Option<LexicalData> {
        Some(self.data.get(self.index)?.clone())
    }
}

impl IntoIterator for Lexical {
    type Item = LexicalData;

    type IntoIter = LexicalDataIter;

    fn into_iter(self) -> Self::IntoIter {
        let data = self.res_list;
        LexicalDataIter { data, index: 0 }
    }
}
