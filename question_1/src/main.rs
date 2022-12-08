use std::io;
fn studentcouncil_votex() {

let yes:bool = true;
let no:bool = false;

println!("Are you a current class rep? (Yes or No)");
let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Invalid answer");
if yes 
{
    println!("Ok.");
    let arr1:[&str;5] = ["100", "200", "300", "400", "500"];
    println!("Which level are you in? {:?}",arr1 );
    for index in 0..5 {
        println!("Numbers {} Level {}",index, arr1[index] );
    }
    let mut input2 = String::new();
    println!("Input number for corresponding level");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let i:i32 = input2.trim().parse().expect("Invalid input");
    if i  == 0{
        println!("Sorry you are not eligible to vote");
    }
    else {
        let mut input3 = String::new();
        println!("What is your CGPA :");
        io::stdin().read_line(&mut input3).expect("Invalid string");
        let cgpa:f32 = input3.trim().parse().expect("Invalid number");

        if cgpa > 4.0{



            let mut input4 = String::new();
            println!("Enter your name");
            io::stdin().read_line(&mut input4).expect("Not a valid string");

             let mut input5 = String::new();
             println!("Enter your email");
             io::stdin().read_line(&mut input5).expect("Not a valid string");

             let mut input6 = String::new();
             println!("Enter your department");
             io::stdin().read_line(&mut input6).expect("Not a valid string");

            let mut input7 = String::new();
    println!("Enter your state of origin");
    io::stdin().read_line(&mut input7).expect("Not a valid string");

    println!("You are {}, from {}, faculty: {}, email: {}",input4, input7, input6, input5 );
    println!("You can vote");
        }
    }

}else{ println!("You can not vote");}





}

fn facpub() {
    let mut input1 = String::new();
    println!("Enter your name");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    let mut input2 = String::new();
    println!(" {}, please enter the number of papers your faculty has published.", input1);
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let a:f64 = input2.trim().parse().expect("Invalid number");

    if a >= 3.0 && a <= 5.0
    {
        println!("Your incentive is N500,000");
    }
    else if a > 5.0 && a < 10.0
    {
        println!("Your incentive is N800,000");
    }
    else if a >= 10.0
    {
        println!("Your incentive is N1,000,000");
    }
    else if a < 3.0
    {
        println!("Your incentive is N100,000");
    }
}
