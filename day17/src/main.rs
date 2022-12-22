enum Shape {
    Row = 0,
    Plus = 1,
    InvertedL = 2,
    Pillar = 3,
    Square = 4,
}

impl Shape {
    fn from_index(index: usize) -> Shape {
        match index % 5 {
            0 => Shape::Row,
            1 => Shape::Plus,
            2 => Shape::InvertedL,
            3 => Shape::Pillar,
            4 => Shape::Square,
            _ => panic!("Invalid shape index"),
        }
    }

    fn get_width(&self) -> usize {
        match self {
            Shape::Row => 4,
            Shape::Plus => 3,
            Shape::InvertedL => 3,
            Shape::Pillar => 1,
            Shape::Square => 2,
        }
    }

    fn get_height(&self) -> usize {
        match self {
            Shape::Row => 1,
            Shape::Plus => 3,
            Shape::InvertedL => 3,
            Shape::Pillar => 4,
            Shape::Square => 2,
        }
    }

    fn calculate_jet_offset(
        &self,
        jet: char,
        current_position: usize,
        chamber_width: usize,
    ) -> usize {
        match jet {
            '>' => {
                if current_position + self.get_width() < chamber_width {
                    current_position + 1
                } else {
                    current_position
                }
            }
            '<' => {
                if current_position > 0 {
                    current_position - 1
                } else {
                    current_position
                }
            }
            _ => panic!("Invalid jet {}", jet),
        }
    }

    fn get_row(&self, shape_row: usize, shape_offset_x: usize, chamber_width: usize) -> String {
        let mut row = " ".repeat(chamber_width);

        match self {
            Shape::Row => match shape_row {
                0 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 4, "####");
                }
                _ => panic!("Invalid shape row"),
            },

            Shape::Plus => match shape_row {
                0 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 3, " # ");
                }
                1 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 3, "###");
                }
                2 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 3, " # ");
                }
                _ => panic!("Invalid shape row"),
            },

            Shape::InvertedL => match shape_row {
                0 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 3, "###");
                }
                1 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 3, "  #");
                }
                2 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 3, "  #");
                }
                _ => panic!("Invalid shape row"),
            },

            Shape::Pillar => match shape_row {
                0 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 1, "#");
                }
                1 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 1, "#");
                }
                2 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 1, "#");
                }
                3 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 1, "#");
                }
                _ => panic!("Invalid shape row"),
            },

            Shape::Square => match shape_row {
                0 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 2, "##");
                }
                1 => {
                    row.replace_range(shape_offset_x..shape_offset_x + 2, "##");
                }
                _ => panic!("Invalid shape row"),
            },
        }

        row
    }
}

fn is_collision(shape: &Shape, shape_x: usize, shape_y: usize, chamber: &Vec<String>) -> bool {
    let chamber_width = chamber.get(0).unwrap().len();

    for shape_row_index in 0..shape.get_height() {
        let chamber_row_to_check = shape_y + shape_row_index;

        let row = chamber.get(chamber_row_to_check);

        if row.is_none() {
            break;
        }

        let row = row.unwrap();
        let shape_row = shape.get_row(shape_row_index, shape_x, chamber_width);

        for col_index in 0..chamber_width {
            let chamber_col = row.get(col_index..col_index + 1).unwrap();
            let shape_col = shape_row.get(col_index..col_index + 1).unwrap();

            if chamber_col == "#" && shape_col == "#" {
                return true;
            }
        }
    }

    false
}

fn draw_shape_at(
    shape: &Shape,
    shape_x: usize,
    shape_y: usize,
    chamber: &mut Vec<String>,
    chamber_width: usize,
) {
    for shape_row_index in 0..shape.get_height() {
        let chamber_row_to_draw = shape_y + shape_row_index;

        if chamber.len() <= chamber_row_to_draw {
            chamber.push(" ".repeat(chamber_width));
        }

        let row = chamber.get_mut(chamber_row_to_draw).unwrap();
        let shape_row = shape.get_row(shape_row_index, shape_x, chamber_width);

        for col_index in 0..chamber_width {
            let chamber_col = row.get(col_index..col_index + 1).unwrap();
            let shape_col = shape_row.get(col_index..col_index + 1).unwrap();

            if chamber_col == " " && shape_col == "#" {
                row.replace_range(col_index..col_index + 1, shape_col);
            }
        }
    }
}

fn main() {
    let file_contents =
        std::fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let jets: Vec<char> = file_contents
        .chars()
        .filter(|q| q == &'>' || q == &'<')
        .collect();

    let mut chamber: Vec<String> = vec!["#######".to_string()];
    let chamber_width = 7;

    let mut shape_index = 0;
    let mut current_step = 0;

    let mut cycle_found = false;
    let mut height_per_cycle = 0;
    let mut next_cycle_stop = 0;
    let mut shape_index_at_cycle_start = 0;
    let mut extra_height = 0;

    loop {
        let shape = Shape::from_index(shape_index);

        let mut shape_x = 2;
        let mut shape_y = chamber.len() + 3;

        loop {
            if is_collision(&shape, shape_x, shape_y, &chamber) {
                draw_shape_at(&shape, shape_x, shape_y + 1, &mut chamber, chamber_width);
                break;
            }

            let jet = jets[current_step % jets.len()];
            let shape_x_before = shape_x;
            shape_x = shape.calculate_jet_offset(jet, shape_x, chamber_width);

            if is_collision(&shape, shape_x, shape_y, &chamber) {
                shape_x = shape_x_before;
            }

            current_step += 1;
            shape_y -= 1;
        }

        if cycle_found && chamber.len() == next_cycle_stop {
            let shapes_per_cycle = shape_index - shape_index_at_cycle_start;

            let target_shape_index = 1000000000000;

            let current = target_shape_index - shape_index;

            let cycles_needed = current / shapes_per_cycle;

            extra_height = cycles_needed * height_per_cycle;
            shape_index = shape_index + cycles_needed * shapes_per_cycle;
        }

        // fast forward if a cycle is found
        if !cycle_found && chamber.len() > 60 {
            // get 30 chamber rows
            let rows = chamber.get(chamber.len() - 30..).unwrap();

            for i in 0..(chamber.len() - 30) {
                // compare
                let mut all_same = true;

                for j in 0..30 {
                    if rows[j] != chamber[i + j] {
                        all_same = false;
                        break;
                    }
                }

                if all_same {
                    shape_index_at_cycle_start = shape_index;

                    height_per_cycle = (chamber.len() - 30) - i;

                    next_cycle_stop = chamber.len() + height_per_cycle;

                    cycle_found = true;
                    break;
                }
            }
        }

        shape_index += 1;

        if shape_index == 2022 {
            println!("Part1: {}", chamber.len() - 1);
        }

        if shape_index == 1000000000000 {
            println!("Part2: {}", chamber.len() - 1 + extra_height);
            break;
        }
    }
}
