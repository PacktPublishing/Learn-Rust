enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn print_light_state(&self) {
        match self {
            TrafficLight::Red => println!("The light is Red. Stop!"),
            TrafficLight::Yellow => println!("The light is Yellow. Caution!"),
            TrafficLight::Green => println!("The light is Green. Go!"),
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    red_light.print_light_state();
    yellow_light.print_light_state();
    green_light.print_light_state();
}