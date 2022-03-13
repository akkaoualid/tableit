pub mod tableformat;

use std::fmt::{self, Write};
use tableformat::{TableFormat, FORMATS};
pub struct Table {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    format: TableFormat,
    widths: Vec<usize>,
}

// TODO:
// support custom formats
// support colors as a feature
// load from config files
// publish to github & crates.io

impl Table {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            widths: Vec::new(),
            format: FORMATS.with(|m| m.borrow()["rst"].clone()),
        }
    }

    pub fn with_format(format: TableFormat) -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            widths: Vec::new(),
            format,
        }
    }

    pub fn add_column<T: ToString>(&mut self, name: T) {
        let name = name.to_string();
        self.widths.push(name.len() + 2);
        self.columns.push(name);
    }

    pub fn add_columns<T: ToString>(&mut self, name: Vec<T>) {
        for n in name {
            self.add_column(n);
        }
    }

    pub fn get_column(&self, index: usize) -> Option<&String> {
        self.columns.get(index)
    }

    pub fn add_row<T: ToString>(&mut self, row: Vec<T>) {
        let it = row.iter().map(|e| e.to_string());

        it.clone().enumerate().for_each(|(i, s)| {
            let w = s.len() + 2;
            if let Some(prev) = self.widths.get_mut(i) {
                if *prev < w {
                    *prev = w
                }
            }
        });

        self.rows.push(it.collect::<Vec<String>>());
    }

    fn format_columns(&self) -> Result<String, fmt::Error> {
        let mut out = String::new();

        let top = self
            .widths
            .iter()
            .map(|w| self.format.column.line.repeat(*w))
            .collect::<Vec<String>>()
            .join(self.format.column.top_separator);
        write!(
            out,
            "{}{}{}",
            self.format.column.top_right_corner, top, self.format.column.top_left_corner
        )?;
        write!(out, "\n")?;

        write!(out, "{}", self.format.column.separator)?;
        for (index, column) in self.columns.iter().enumerate() {
            write!(
                out,
                "{c:^n$}{s}",
                c = column,
                n = self.widths[index],
                s = self.format.column.separator
            )?;
        }
        write!(out, "\n")?;

        let bot = self
            .widths
            .iter()
            .map(|w| self.format.column.line.repeat(*w))
            .collect::<Vec<String>>()
            .join(self.format.column.mid_separator);
        write!(
            out,
            "{}{}{}",
            self.format.column.bot_right_corner, bot, self.format.column.bot_left_corner
        )?;
        write!(out, "\n")?;

        Ok(out)
    }

    fn format_rows(&self) -> Result<String, fmt::Error> {
        let mut out = String::new();
        for row in self.rows.iter() {
            write!(out, "{}", self.format.row.separator)?;
            for i in 0..(self.columns.len()) {
                write!(
                    out,
                    "{c:^n$}{s}",
                    c = row.get(i).unwrap_or(&" ".to_string()),
                    n = self.widths[i],
                    s = self.format.row.separator
                )?;
            }
            write!(out, "\n")?;
        }

        let bot = self
            .widths
            .iter()
            .map(|w| self.format.row.line.repeat(*w))
            .collect::<Vec<String>>()
            .join(self.format.row.bot_separator);
        write!(
            out,
            "{}{}{}",
            self.format.row.bot_right_corner, bot, self.format.row.bot_left_corner
        )?;
        write!(out, "\n")?;
        Ok(out)
    }

    pub fn render(&self) -> Result<String, fmt::Error> {
        let mut out = String::new();
        write!(out, "{}", self.format_columns()?)?;
        write!(out, "{}", self.format_rows()?)?;
        Ok(out)
    }
}
