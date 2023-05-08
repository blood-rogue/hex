use std::io::{BufWriter, Write};

const HORIZONTAL_BAR: &str = "─";

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            rows: Vec::new(),
            headers,
        }
    }

    pub fn insert(&mut self, row: Vec<String>) {
        self.rows.push(row)
    }

    fn calc_max(&self) -> Vec<usize> {
        let mut maxes: Vec<_> = self.headers.iter().map(String::len).collect();

        for row in &self.rows {
            for (max, data) in maxes.iter_mut().zip(row.iter()) {
                if data.len() > *max {
                    *max = data.len()
                }
            }
        }

        maxes.iter().map(|max| max + 2).collect()
    }

    pub fn display(self) {
        let mut writer = BufWriter::with_capacity(16384, std::io::stdout().lock());
        let maxes = self.calc_max();
        let (&last_max, rest_maxes) = maxes.split_last().unwrap();

        write!(writer, "╭").unwrap();
        for &max in rest_maxes {
            write!(writer, "{}", HORIZONTAL_BAR.repeat(max)).unwrap();
            write!(writer, "┬").unwrap();
        }

        write!(writer, "{:last_max$}", HORIZONTAL_BAR.repeat(last_max)).unwrap();
        writeln!(writer, "╮").unwrap();

        write!(writer, "│").unwrap();
        for (i, data) in self.headers.iter().enumerate() {
            write!(writer, "{:^max$}│", data, max = maxes[i]).unwrap();
        }
        writeln!(writer).unwrap();

        write!(writer, "├").unwrap();
        for (j, _) in self.headers.iter().enumerate() {
            write!(writer, "{}", HORIZONTAL_BAR.repeat(maxes[j])).unwrap();
            if j != self.headers.len() - 1 {
                write!(writer, "┼").unwrap();
            }
        }
        writeln!(writer, "┤").unwrap();

        for row in self.rows.iter() {
            write!(writer, "│").unwrap();
            for (i, data) in row.iter().enumerate() {
                write!(writer, "{data:^max$}│", max = maxes[i]).unwrap();
            }
            writeln!(writer).unwrap();
        }

        write!(writer, "╰").unwrap();
        for &max in rest_maxes {
            write!(writer, "{}", HORIZONTAL_BAR.repeat(max)).unwrap();
            write!(writer, "┴").unwrap();
        }

        write!(writer, "{:last_max$}", HORIZONTAL_BAR.repeat(last_max)).unwrap();
        writeln!(writer, "╯").unwrap();
    }
}
