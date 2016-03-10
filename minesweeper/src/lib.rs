struct Board {
    pieces: Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
}

impl Board {
    // For each row, check annotated row
    fn annotated(&self) -> Vec<String> {
        (0..self.num_rows).map(|y| self.annotated_row(y)).collect()
    }

    // For each space (non mine cell), check neighbouring points, write '*' for mines
    fn annotated_row(&self, y: usize) -> String {
        self.pieces[y]
            .iter()
            .enumerate()
            .map(|(x, &c)| {
                if c == ' ' {
                    self.count_neighbouring_mines_char(x, y)
                } else {
                    c
                }
            })
            .collect::<String>()
    }

    // count_neighbouring_mines_chartakes an cell defined for its x and y position in a matrix
    // and returns its type: mine, space or number
    fn count_neighbouring_mines_char(&self, x: usize, y: usize) -> char {
        let mut count = 0;
        // Iterate from left to right
        for x1 in neighbouring_points(x, self.num_cols) {
            for y1 in neighbouring_points(y, self.num_rows) {
                let piece = self.pieces[y1][x1];
                if piece == '*' {
                    count += 1;
                }
            }
        }
        if count == 0 {
            ' '
        } else {
            (('0' as u8) + count) as char
        }
    }
}

pub fn annotate(pieces: &[&str]) -> Vec<String> {
    // Flatten mines
    let pieces_vec = pieces.iter().map(|&r| r.chars().collect()).collect();

    Board {
        pieces: pieces_vec,
        num_rows: pieces.len(),
        num_cols: pieces[0].len(),
    }
    .annotated()
}

// neighbouring_points gives, for an array position 'x' and a defined size 'limit'
// the neighbour values in a vec. So for:
// 0, 5 => Gives 0,1
// 1, 5 => Gives 1,0,2
// 2, 5 => Gives 2,1,3
// 3, 5 => Gives 3,2,4
// 4, 5 => Gives 4,3
pub fn neighbouring_points(x: usize, limit: usize) -> Vec<usize> {
    let mut offsets = vec![x];
    if x >= 1 {
        offsets.push(x - 1);
    }
    if x + 2 <= limit {
        offsets.push(x + 1);
    }
    offsets
}
