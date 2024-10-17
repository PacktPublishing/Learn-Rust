use std::collections::VecDeque;

#[derive(Debug)]
struct Snake {
    body: VecDeque<(i32, i32)>,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    fn new(initial_position: (i32, i32)) -> Self {
        let mut body = VecDeque::new();
        body.push_back(initial_position); // Add the head as the initial position
        Snake { body }
    }

    fn mv(&mut self, direction: Direction, grow: bool) {
        let (head_x, head_y) = *self.body.front().expect("Snake must have a head");

        let new_head = match direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        self.body.push_front(new_head);

        if !grow {
            self.body.pop_back();
        }
    }

    fn print_body(&self) {
        println!("Snake body: {:?}", self.body);
    }
}

fn main() {
    let mut snake = Snake::new((5, 5));

    snake.print_body();

    snake.mv(Direction::Right, false);  // Move right
    snake.print_body();                 // Should move to (6, 5)

    snake.mv(Direction::Down, false);   // Move down
    snake.print_body();                 // Should move to (6, 6)

    snake.mv(Direction::Left, true);    // Move left, grow the snake
    snake.print_body();                 // Should move to (5, 6) and grow

    snake.mv(Direction::Up, false);     // Move up
    snake.print_body();                 // Should move to (5, 5) and not grow
}
