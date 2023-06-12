use crate::SearchDirection;
use crate::highlighting;

use std::{cmp, fmt::format};
use unicode_segmentation::UnicodeSegmentation;
use termion::color;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
    highlighting: Vec<highlighting::Type>,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.grapheme_indices(true).count(),
            highlighting: Vec::new(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start,end);

        let mut result = String::new();
        #[allow(clippy::integer_arithmetic)]
        for (index, grapheme) in self.string[..].graphemes(true).enumerate().skip(start).take(end-start) {
            if let Some(c) = grapheme.chars().next() {
                let highlighting = self.highlighting.get(index).unwrap_or(&highlighting::Type::None);
                let hightlight_color = format!("{}", color::Fg(highlighting.to_color()));
                result.push_str(&hightlight_color[..]);
                if c == '\t' {
                    result.push_str("  ");
                } else {
                    result.push_str(grapheme);
                }
                let hightlight_reset = format!("{}", color::Fg(color::Reset));
                result.push_str(&hightlight_reset[..]);
            }
        }
        result
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }

    pub fn delete(&mut self, at: usize) {
        if at > self.len() {
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.string = result;
        self.len = length;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut row = String::new();
        let mut row_length = 0;
        let mut new_row = String::new();
        let mut new_row_length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                row.push_str(grapheme);
                row_length += 1;
            } else {
                new_row.push_str(grapheme);
                new_row_length += 1;
            }
        }

        self.string = row;
        self.len = row_length;
        Self {
            string: new_row,
            len: new_row_length,
            highlighting: Vec::new(),
        }
    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len {
            return None;
        }

        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };

        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };


        let substring: String = self.string[..].graphemes(true).skip(start).take(end - start).collect();
        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };

        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in
                substring[..].grapheme_indices(true).enumerate() 
            {
                if matching_byte_index == byte_index {
                    #[allow(clippy::integer_arithmetic)]
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }

    pub fn hightlight(&mut self) {
        let mut highlighting: Vec<highlighting::Type> = Vec::new();
        for c in self.string.chars() {
            if c.is_ascii_digit() {
                highlighting.push(highlighting::Type::Number);
            } else {
                highlighting.push(highlighting::Type::None);
            }
        }
        self.highlighting = highlighting;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}