use std::fs::File;
use std::io::{self, Write};

struct StaffMember {
    name: String,
    department: String,
    services: String,
    qualification: String,
    code: u8,
}

fn code_7(member: &StaffMember) -> io::Result<()> {
    let mut file = File::create(format!("{}_{}.txt", member.name.to_lowercase().replace(" ", "_"), member.code))?;
    writeln!(file, "Department: {}", member.department)?;
    writeln!(file, "Services: {}", member.services)?;
    Ok(())
}

fn code_8(member: &StaffMember) -> io::Result<()> {
    let mut file = File::create(format!("{}_{}.txt", member.name.to_lowercase().replace(" ", "_"), member.code))?;
    writeln!(file, "Department: {}", member.department)?;
    writeln!(file, "Services: {}", member.services)?;
    Ok(())
}

fn code_9(member: &StaffMember) -> io::Result<()> {
    let mut file = File::create(format!("{}_{}.txt", member.name.to_lowercase().replace(" ", "_"), member.code))?;
    writeln!(file, "Department: {}", member.department)?;
    writeln!(file, "Services: {}", member.services)?;
    Ok(())
}


// More functions can be done for more staff members

fn main() -> io::Result<()> {
    let staff = vec![
        StaffMember {
            name: "Aigbona Juliet".to_string(),
            department: "Consulting".to_string(),
            services: "\nAnalytics consulting serivces, \nCustomer experience, Cybersecurity, strategy, risk, compliance and resilience; \nDigital transformation, \nRisk consulting services, \nSupply chain and operations, \nTechnology transformation".to_string(),
            qualification: "B.Sc.".to_string(),
            code: 7,
        },
        StaffMember {
            name: "Ehis Ero".to_string(),
            department: "Strategy".to_string(),
            services: "\nStrategy consulting, \nCorporate and growth strategy, \nTransaction strategy and execution, \nRestructuring and turnaround strategy, \nIndustry strategy, \nDigital business building, \nCommercial strategy".to_string(),
            qualification: "M.Sc.".to_string(),
            code: 9,
        },
        StaffMember {
            name: "Adamu Sagamu".to_string(),
            department: "Tax".to_string(),
            services: "\nTax planning, \nTax function operations, \nTax policy and controversy, \nGlobal trade, \nTax accounting, \nTax compliance, \nTransaction tax".to_string(),
            qualification: "B.Sc.".to_string(),
            code: 8,
        },
        StaffMember {
            name: "Akpevwe Iloka".to_string(),
            department: "Assurance".to_string(),
            services: "\nAudit services, \nClimate change and sustainability services, \nFinancial accounting advisory services, \nForensic and integrity services, \nPrivate client audit experience, \nAccounting Link, \nAssurance".to_string(),
            qualification: "HND".to_string(),
            code: 7,
        },
        StaffMember {
            name: "Maria Akinsola".to_string(),
            department: "Transaction and corporate finance".to_string(),
            services: "\nCorporate finance, \nDivestments and carve-outs, \nSustainability and ESG Services, \nM&A advisory, \nM&A integration, \nM&A technology and tools, \nM&A advanced analytics".to_string(),
            qualification: "M.Sc.".to_string(),
            code: 9,
        },
        StaffMember {
            name: "Gbenga Daniels".to_string(),
            department: "People and workforce".to_string(),
            services: "\nChange management and experience, \nHR transformation, \nIntegrated workforce mobility, \nLearning and development consulting, \nRecognition and reward advisory, \nWorkforce analytics, \nPeople and workforce".to_string(),
            qualification: "HND".to_string(),
            code: 8,
        },
        // Add more staff members here
    ];

    for member in staff {
        match member.code {
            7 => code_7(&member)?,
            8 => code_8(&member)?,
            9 => code_9(&member)?,
            // Add cases for other codes here
            _ => println!("Invalid code for staff member {}", member.name),
        }
    }

    Ok(())
}
