use std::collections::VecDeque;

fn main() {
    let mut deque: VecDeque<i32> = VecDeque::new();

    deque.push_front(1);
    deque.push_front(2);
    deque.push_front(3);

    deque.push_back(4);
    deque.push_back(5);
    deque.push_back(6);

    deque.pop_front();
    deque.pop_back();

    println!("{:?}", deque);
}
