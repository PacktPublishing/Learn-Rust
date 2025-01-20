use std::rc::Rc;

fn main() {
    let rc1 = Rc::new(5);
    println!("Reference count: {}", Rc::strong_count(&rc1));

    let _rc2 = Rc::clone(&rc1);
    println!("Reference count: {}", Rc::strong_count(&rc1));

    {
        let _rc3 = Rc::clone(&rc1);
        println!("Reference count: {}", Rc::strong_count(&rc1));
    }

    println!("Reference count after rc3 is out of scope: {}", Rc::strong_count(&rc1));
}