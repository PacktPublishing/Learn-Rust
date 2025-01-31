struct Holder {
    data: &'static str,
}

struct StaticHolder {
    data: &'static str,
}

fn main() {
    let holder = Holder {
        data: "static text"
    };
    println!("{}", holder.data);

    // This will not compile
    // let heap_data = "heap text".to_string();
    // let holder = Holder {
    //     data: &heap_data
    // };
    // println!("{}", holder.data);
}
