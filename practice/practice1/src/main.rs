use std::fs::File;
use std::io::{self, Write};

struct StaffMember {
    name: String,
    department: String,
    services: String,
    qualification: String,
    code: u8,
}

fn main() -> io::Result<()> {
    // Create a list of staff members
    let staff = vec![
        StaffMember {
            name: "Aigbona Juliet".to_string(),
            department: "Consulting".to_string(),
            services: "Analytics consulting serivces, Customer experience, Cybersecurity, strategy, risk, compliance and resilience; Digital transformation, Risk consulting services, Supply chain and operations, Technology transformation".to_string(),
            qualification: "B.Sc.".to_string(),
            code: 7,
        },
        StaffMember {
            name: "Ehis Ero".to_string(),
            department: "Strategy".to_string(),
            services: "Corporate and growth strategy, Customer experience, HR transformation".to_string(),
            qualification: "M.Sc.".to_string(),
            code: 9,
        },
        // Add more staff members here...
    ];

    // Iterate through the list of staff members
    for member in staff {
        // Create a file with the staff member's name
        let mut file = File::create(format!("{}_{}.txt", member.name.to_lowercase().replace(" ", "_"), member.code))?;

        // Write the department and corresponding services to the file
        writeln!(file, "Department: {}", member.department)?;
        writeln!(file, "Services: {}", member.services)?;
    }

    Ok(())
}
