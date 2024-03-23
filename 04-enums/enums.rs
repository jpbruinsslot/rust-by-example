// Title: Enums
// Language: Rust
//
// Enums lets you express a type that can be one of a few different variants.
// Rust's enums offer some powerful features, such as the ability to attach
// data to each variant (known as "associated data" or "payloads") and the
// ability to define methods on enums. Enums are particularly useful when you
// have a fixed set options that you want to express in your code.

fn main() {

    // Define an enum named CardinalDirection with four variants. A variant is
    // assigned an integer value by default starting from 0.
    enum CardinalDirection {
        North,
        East,
        South,
        West,
    }

    // To create an instances of the `CardinalDirection` enum
    let north = CardinalDirection::North;
    let east = CardinalDirection::East;
    let south = CardinalDirection::South;
    let west = CardinalDirection::West;

    // Enums are namespaced under its identifier, all four variants are the
    // same type `CardinalDirection`. That allows us to define functions that
    // take `CardinalDirection` as an argument:
    fn move_to(direction: CardinalDirection) {
        match direction {
            CardinalDirection::North => println!("Move north"),
            CardinalDirection::East => println!("Move east"),
            CardinalDirection::South => println!("Move south"),
            CardinalDirection::West => println!("Move west"),
        }
    }

    // Call the `move_to` function with the variants of the `CardinalDirection`
    move_to(north);
    move_to(east);
    move_to(south);
    move_to(west);

    // Enums can also have associated data. This is useful when you want to store
    // additional information about an enum variant.
    enum Animal {
        Dog(String),
        Cat { name: String, age: u8 },
        Bird,
    }

    // To create an instance of the `Animal` enum we can do
    let dog = Animal::Dog(String::from("Sam"));

    // The `Cat` variant includes a name and age, so we need to provide values
    let cat = Animal::Cat {
        name: String::from("Tommy"),
        age: 15,
    };

    // The `Bird` variant does not include any data, so we can create an instance
    let bird = Animal::Bird;

    // Enums can also have methods defined on them.
    impl Animal {
        // A method that returns the name of the animal.
        fn name(&self) -> String {
            match self {
                Animal::Dog(name) => name.clone(),
                Animal::Cat { name, .. } => name.clone(),
                Animal::Bird => String::from("Bird"),
            }
        }

        fn age(&self) -> u8 {
            match self {
                Animal::Cat { age, .. } => *age,
                _ => 0,
            }
        }
    }

    println!("The name of the dog is: {}", dog.name());
    println!(
        "The name of the cat is: {}, and its age is: {}",
        cat.name(),
        cat.age(),
    );
    println!("The name of the bird is: {}", bird.name());
}
