pub(crate) fn read_until(s: &str, b: usize, f: fn(c: char) -> bool) -> Option<usize> {
    if b >= s.len() {
        return None;
    }

    let cs = s[b..].char_indices();
    for (i, c) in cs {
        if f(c) {
            return Some(b + i);
        }
    }

    Some(s.len())
}

#[derive(Debug)]
pub(crate) struct Matrix {
    pub data: Vec<String>,
    pub width: i64,
    pub height: i64,
}

impl Matrix {
    pub fn new(input: &str) -> Matrix {
        let data: Vec<_> = input
            .split('\n')
            .filter(|l| l.len() > 0)
            .map(|s| s.to_string())
            .collect();
        let width = data[0].len() as i64;
        let height = data.len() as i64;

        Matrix {
            data,
            width,
            height,
        }
    }

    pub fn get_point(&self, x: i64, y: i64) -> Option<&str> {
        if x > self.width - 1 || x < 0 {
            return None;
        }

        if y > self.height - 1 || y < 0 {
            return None;
        }

        Some(&self.data[y as usize][x as usize..((x as usize) + 1)])
    }

    pub fn get_point_char(&self, x: i64, y: i64) -> Option<char> {
        match self.get_point(x, y) {
            Some(s) => Some(s.chars().nth(0).unwrap()),
            None => None,
        }
    }

    pub fn get_line(&self, y: i64) -> Option<&str> {
        if y > self.height - 1 || y < 0 {
            return None;
        }

        Some(&self.data[y as usize])
    }
}
