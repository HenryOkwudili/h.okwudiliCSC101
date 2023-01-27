use std::io;
use std::io::Read;

fn access_levels() {
    println!("Welcome, PLease select your access level.");

    // Level allocation vectors
    let level = vec!["Administrator", "Manager", "Employee", "Customer", "Vendor"];
    let num = vec![1, 2, 3, 4, 5];
    for i in 0..num.len()
    {
        print!("{} for {}\n",num[i],level[i] );
    }

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1:i32 = input1.trim().parse().expect("Invalid input");
    println!("{}",input1 );


    if input1 == 1
    {
        let mut file1 = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents1 = String::new();
        file1.read_to_string(&mut contents1).unwrap();
        print!("{}",contents1);

    }

    else if input1 == 2
    {

        let mut file2 = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents2 = String::new();
        file2.read_to_string(&mut contents2).unwrap();
        print!("{}",contents2);
    }

    else if input1 == 3
    {

        let mut file3 = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents3 = String::new();
        file3.read_to_string(&mut contents3).unwrap();
        print!("{}",contents3);
    }
    else if input1 == 4
    {

        let mut file4 = std::fs::File::open("customer_table.sql").unwrap();
        let mut contents4 = String::new();
        file4.read_to_string(&mut contents4).unwrap();
        print!("{}",contents4);
    }
    else if input1 == 5
    {

        let mut file5 = std::fs::File::open("dataplan_table.sql").unwrap();
        let mut contents5 = String::new();
        file5.read_to_string(&mut contents5).unwrap();
        print!("{}",contents5);
    }

}

fn main() {
    access_levels()
}
