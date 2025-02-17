use std::io;

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
        fah = input2 * 1.8 + 32.0;
        kel = input2 + 273.15;
        println!("{}C = {}F", input2, fah);
        println!("{}C = {}K", input2, kel);
        println!("Program now exiting");
    }

    else if input == "2"{
        cel = input2 - 32.0;
        let cel2: f32;
        let cel3: f32;
        cel2 = cel * 5.0;
        cel3 = cel2 / 9.0;
        kel = cel3 + 273.15;
        println!("{}F = {}C", input2, cel3);
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
