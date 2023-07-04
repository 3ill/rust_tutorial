/**
 * !Topic: Implementing functionality with the impl keyword
 * ?Requirements: Print the characteristics of a shipping box
 * ? Must include dimensions, weight and color
 *
 * todo: use a structure to encapsulate the box characteristics
 * todo: use an enum for the box color
 * todo: implement functionality on the box to create a new box
 * todo: implement functionality on the box struct to print the characteristic
 */

//? create an enum for box colors
enum Tint {
    Blue,
    Green,
    Black,
}

//? create a dimensions struct
struct Dimensions {
    weight: f64,
    height: f64,
    depth: f64,
}

//? create a shipping Box struct
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Tint,
}

//? implementation to create new box
impl Box {
    //? this function creates a new Box of the implementation type
    fn create_box() -> Self {
        Self {
            dimensions: Dimensions {
                weight: 64.0,
                height: 32.0,
                depth: 15.0,
            },
            weight: 64.0,
            color: Tint::Black,
        }
    }

    //? This function creates a new box by accepting variables
    fn create_new_box(h: f64, dp: f64, kg: f64, color: Tint) -> Self {
        Self {
            dimensions: Dimensions {
                weight: kg,
                height: h,
                depth: dp,
            },
            weight: kg,
            color: color,
        }
    }

    //? function to print box characteristics
    fn show_box_details(&self) {
        match self.color {
            Tint::Black => println!("You chose a black box"),
            Tint::Blue => println!("You chose a blue box"),
            Tint::Green => println!("You chose a green box"),
        }

        println!(
            "Box characteristics =>  {:?} kg, {:?} height and , {:?} depth",
            self.weight, self.dimensions.height, self.dimensions.depth
        );
    }
}

fn main() {
    let new_box = Box::create_box();
    new_box.show_box_details();

    let order = Box::create_new_box(12.0, 15.0, 8.0, Tint::Green);
    order.show_box_details();

    let order2 = Box::create_new_box(12.0, 89.0, 50.0, Tint::Blue);
    order2.show_box_details();
}
