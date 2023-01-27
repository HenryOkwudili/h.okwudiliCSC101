

fn main() {

     let _lager:[&str;6] = ["33 Export", "Desparados", "Goldberg", "Gulder", "Heineken", "Star"];
     let _stout:[&str;6] = ["Legend", "Turbo King", "Williams", " ", " ", " "];
     let _non_alcoholic:[&str;6] = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", " ", " "];

     for index in 0..6 {
        println!("{} {} {} {}",index,_lager[index],_stout[index],_non_alcoholic[index]);
     }
        
     
}


