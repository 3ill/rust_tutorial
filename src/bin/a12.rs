/**
 * !Topic: Implementation
 */

struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    //? to create a new type of the implementation type, the Self keyword is used
    fn freeze() -> Self {
        Self { degrees_f: 32.0 }
    }

    //? when returning a Self, parameters can still be assigned
    fn boiling(temp: f64) -> Self {
        Self { degrees_f: temp }
    }
    //? using the keyword self on this function denotes that it accepts a parameter of the implementation type
    //? in this case the implementation type is the Temperature type
    fn show_temp(&self) {
        println!("{:?} degree F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.0 };
    //? to call a function that accepts a &self parameter,you use a dot notation
    hot.show_temp();

    //? to call a function that doesnt accept a &self parameter in an implementation, you use a :: like an enum
    let cold = Temperature::freeze();
    cold.show_temp();

    let hot = Temperature::boiling(101.5);
    hot.show_temp();
}
