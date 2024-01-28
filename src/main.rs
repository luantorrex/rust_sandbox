fn add(a: i32, b:i32) -> i32 {
   return a + b;
}

fn get_name_and_age() -> (String, i32) {
    let name = String::from("Pedro");
    let age = 30;
    return (name, age);
}

fn main() {
    let mut counter: i32 = 0;

    // Dynamic String
    let dynamic_string = String::from("Pair");

    // String Slice (immutable)
    let static_string: &str = "Odd";

    while counter < 10 {
       counter += 1;

       if counter % 2 == 0 {
           println!("{}: {} | {}", counter, dynamic_string, add(counter, counter));
       }
       else {
           println!("{}: {} | {}", counter, static_string, add(counter, counter));
       }
    }

    println!("--------------------------------");

    let(name, age) = get_name_and_age();
    println!("Name: {}, Age: {}", name, age);

    println!("--------------------------------");

    // Mutable Array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Slice (Immutable)
    let slice: &[i32] = &numbers[1..4];

    for num in slice{
        println!("Print array: {}", num)
    }
}
