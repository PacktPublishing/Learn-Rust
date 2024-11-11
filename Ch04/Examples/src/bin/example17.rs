#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let traffic_light = TrafficLight::Red;

    if traffic_light == TrafficLight::Red {
        println!("The light is red. Please stop.");
    }
}
