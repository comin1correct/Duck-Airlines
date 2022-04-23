#![allow(unused_variables)]

fn main() {
    _haversine_formula()
}

fn _match(){
    // let word = "Dog";
    // if word == "Duck"{
    //     println!("Quack");
    // } else if word == "Dog" {
    //     println!("Bark");
    // }
    // else {
    //     println!("all quiet out here");
    // }

    let animal = "Dog";
    match animal {
        "Duck" => println!("Quack"),
        "Dog" => println!("Bark"),
        _ => println!("All quite out here")
    };

}

fn _option(){
    // no null data type
    // option is an enumeration that has two values: Some & None
    let phrase = String::from("Duck Airlines");
    let letter = phrase.chars().nth(15);

    let value = match letter{
        Some(character) => character.to_string(),
        None => String::from("No value")
    };

    println!("{}", value);
}

fn _enum(){
    enum NavigationAids{
        NDB,
        VOR ,
        VORDME,
        //FIX {name: String, latitude: f32, longitude: f32}
    }

    println!("NDB:\t{}", NavigationAids::NDB as u8);
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);
}

fn _control_flow(){
    let word = "Dog";
    if word == "Duck"{
        println!("Quack");
    } else if word == "Dog" {
        println!("Bark");
    }
    else {
        println!("all quiet out here");
    }
}

fn _haversine_formula(){
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    // Haversine formula. starting and ending point of lat/long + radius and trig calc

    // cleveland
    let kcle_lat_degrees: f64 = 41.4075;
    let kcle_long_degrees: f64 = -81.851111;

    // salt lake city
    let kslc_lat_degrees: f64 = 40.7861;
    let kslc_long_degrees: f64 = 111.9822;

    //convert to radians
    let kcle_lat_radians = kcle_lat_degrees.to_radians();
    let kslc_lat_radians = kslc_lat_degrees.to_radians();

    //difference btw lat and long
    let delta_lat = (kcle_lat_degrees - kslc_lat_degrees).to_radians();
    let delta_long = (kcle_long_degrees - kslc_long_degrees).to_radians();

    //trig calc
    let inner_central_angle = f64::powi((delta_lat / 2.0).sin(), 2)
        + kcle_lat_radians.cos()
        * kslc_lat_radians.cos()
        * f64::powi((delta_long / 2.0).sin(),2);

    let central_angle = 2.0 * inner_central_angle.sqrt().asin();

    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
    let miles = distance * 0.6214;

    println!("The distance between the two points is {:.1} kilometer and {:.1} miles", distance, miles)
}


fn _operators(){
    // calculating exponents
    let squared = i32::pow(8,2);
    let float_integer = f32::powi(6.5,3);
    let float_float = f32::powf(6.5, 3.14);
    println!("integer: {}",squared);
    println!("float to int: {}",float_integer);
    println!("float to float: {}",float_float);

    // logical: equality, negation, AND, OR, < , >, ...
    let are_equal_is_true = 1 == 1;
    let are_equal_is_false = 1 == 2;

    let are_not_equal = 1 != 2;
    let is_true = true;
    let is_false = !is_true;

    let _ = true && true;
    let _ = false || true;


}
fn _scope_shadowing(){
    let scope_test = "outer scope";
    println!("{}", scope_test);
    {
        //using the same var name in a inner scope is called shadowing
        let scope_test = "inner scope";
        println!("{} a", scope_test);
    }
    {
        println!("{} b", scope_test);
    }

    println!("{}", scope_test);
}
fn _variable_mutability(){
    //by default var are immutable for safety, concurrency, speed (optimization).
    // use the mut keyword to override this
    let mut _changeable_variable = 0;
}
fn _casting_data_types(){
    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;
    //no implicit casting handled by the compiler!
    //let result = float_thirty_two/unsigned_eight;

    //we are responsible for changing the data type and any responsibilities that come with that.
    let cast_unsigned_eight = unsigned_eight as f32;
    let result = float_thirty_two/cast_unsigned_eight;
    println!("{}", result);
}
fn _simple_variable(){
    let my_var_name: u32 = 0;
    let my_inferred_var = 0;
}
fn _string_to_string_slice(){
    let person_name_string = String::from("Donald Mallard");

    //getting address where data lives on the heap --> & [a.k.a de-referencing the variable]
    let person_name_slice = &person_name_string;
    let person_name_slice2 = person_name_string.as_str();
}
fn _string_slice_concatenation(){
    let duck = "Duck";
    let airlines = "Airlines";

    // option 1:
    let airline_name = [duck, " ", airlines].concat();
    print!("{}", airline_name);

    //option 2:
    let airline_name_format = format!("\n{} {}", duck, airlines);
    println!("{}", airline_name_format);

    //option 3:
    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";
    println!("\n{}", slogan);
}
fn _array_tuple(){
    //let location: [f32;2] = [41.4094069, -81.8546911];
    let location= ("KCLE", 41.4094069, -81.8546911);
    //write compound data types by index:
    println!("location name: {}, latitude: {}, longitude: {}", location.0, location.1, location.2);

    //destructor array
    let(name, latitude, longitude) = location;
    println!("location name: {}, latitude: {}, longitude: {}", name, latitude, longitude);
}
