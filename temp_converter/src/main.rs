use std::io;

const FREEZING_POINT_FAH: f32 = 273.15;
const KELVIN_OFFSET: f32 = 32.0;
const TEMPERATURE_CONVERSION_RATIO: f32 = 1.8;
fn main() {
    let mut input = String::new();
    
    println!("Please select what you want to convert!");
    println!("Press 1 for Celsius");
    println!("Press 2 for Fahrenheit");
    println!("Press 3 for Kelvin");
    
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    println!("Please enter the number to convert");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let input2 = input2.trim();
    let input2 = input2.parse::<f32>().unwrap();
    let cel: f32;
    let kel: f32;
    let fah: f32;

    if input == "1" {
        fah = cel_to_fah(input2);
        kel = cel_to_kel(input2);
        println!("{}C = {}F", input2, fah);
        println!("{}C = {}K", input2, kel);
        println!("Program now exiting");
    }

    else if input == "2"{
        cel = fah_to_cel(input2);
        kel = fah_to_kel(input2);
        println!("{}F = {}C", input2, cel);
        println!("{}F = {}K", input2, kel);
        println!("Program now exiting");
    }

    else if input == "3"{
        cel = input2 - 273.15;
        fah = cel * 1.8 + 32.0;
        println!("{}K = {}C", input2, cel);
        println!("{}K = {}F", input2, fah);
        println!("Program now exiting");
    }

    else{
        println!("Invalid Input. Exiting Program");
    }

}

fn cel_to_fah(cel: f32) -> f32 {
    return cel * TEMPERATURE_CONVERSION_RATIO + KELVIN_OFFSET;
}
fn cel_to_kel(cel: f32) -> f32 {
    return cel + FREEZING_POINT_FAH;
}
fn fah_to_cel(fah: f32) -> f32{
    return fah - KELVIN_OFFSET * 5.0 / 9.0;
}
fn fah_to_kel(fah: f32) -> f32{
    return fah - KELVIN_OFFSET * 5.0 / 9.0 + FREEZING_POINT_FAH;
}