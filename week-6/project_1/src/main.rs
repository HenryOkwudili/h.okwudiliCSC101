use std::io;

//Functions for the formulas
fn area_of_trapezium() {
    let area = h/2*(a+b);
    println!("The area of the Trapezium = {}",area );

}

fn area_of_rhombus() {
    let area = (1/2)*a*b;
    println!("The area of the rhombus = {}",area );
}

fn area_of_parallelogram() {
    let area = b * a;
    println!("The area of the parallelogram = {}",area );
}

fn area_of_cube() {
    let area = 6 * (a*a);
    println!("The area of the cube = {}",area );
}

fn volume_of_cylinder() {
    let volume = (22/7) * (a*a)*h;
    println!("The volume of cylinder = {}",volume );
}



fn main()
{
    println!()
    let arr1:[&str;5] = ["Trapezium", "rhombus", "parallelogram", "cube", "cylinder"];
    println!("\nformulas are :{:?}", arr1);

    for index in 0..5 {
        println!("Formula number {} solves area/volume for :{}",index, arr1 );
    }

    println!("Select a number for the formula you want to solve");
    
}

