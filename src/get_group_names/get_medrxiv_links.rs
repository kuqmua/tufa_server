use std::collections::HashMap;
pub fn get_medrxiv_links() -> HashMap<&'static str, &'static str> {
    let arxiv_sections_links: HashMap<&str, &str> = [
        (
            "Addiction Medicine",
            // "https://github.com/ryeyeyhh/dfndfnhoj",
            "http://connect.medrxiv.org/medrxiv_xml.php?subject=Addiction_Medicine",
        ),
        (
            "Allergy and Immunology",
            "http://connect.medrxiv.org/medrxiv_xml.php?subject=Allergy_and_Immunology",
        ),
        (
            "Anesthesia",
            "http://connect.medrxiv.org/medrxiv_xml.php?subject=Anesthesia",
        ),
         ("Cardiovascular Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Cardiovascular_Medicine"),
         ("Dentistry and Oral Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Dentistry_and_Oral_Medicine"),
         ("Dermatology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Dermatology"),
         ("Emergency Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Emergency_Medicine"),
         ("Endocrinology","http://connect.medrxiv.org/medrxiv_xml.php?subject=endocrinology"),
         ("Epidemiology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Epidemiology"),
         ("Forensic Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Forensic_Medicine"),
         ("Gastroenterology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Gastroenterology"),
         ("Genetic and Genomic Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Genetic_and_Genomic_Medicine"),
         ("Geriatric Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Geriatric_Medicine"),
         ("Health Economics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Health_Economics"),
         ("Health Informatics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Health_Informatics"),
         ("Health Policy","http://connect.medrxiv.org/medrxiv_xml.php?subject=Health_Policy"),
         ("Health Systems and Quality Improvement","http://connect.medrxiv.org/medrxiv_xml.php?subject=Health_Systems_and_Quality_Improvement"),
         ("Hematology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Hematology"),
         ("HIV/AIDS","http://connect.medrxiv.org/medrxiv_xml.php?subject=hivaids"),//тут чет иногда ничего нет
         ("Infectious Diseases","http://connect.medrxiv.org/medrxiv_xml.php?subject=infectious_diseases"),
         ("Intensive Care and Critical Care Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Intensive_Care_and_Critical_Care_Medicine"),
         ("Medical Education","http://connect.medrxiv.org/medrxiv_xml.php?subject=Medical_Education"),
         ("Medical Ethics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Medical_Ethics"),
         ("Nephrology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Nephrology"),
         ("Neurology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Neurology"),
         ("Nursing","http://connect.medrxiv.org/medrxiv_xml.php?subject=Nursing"),
         ("Nutrition","http://connect.medrxiv.org/medrxiv_xml.php?subject=Nutrition"),
         ("Obstetrics and Gynecology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Obstetrics_and_Gynecology"),
         ("Occupational and Environmental Health","http://connect.medrxiv.org/medrxiv_xml.php?subject=Occupational_and_Environmental_Health"),
         ("Oncology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Oncology"),
         ("Ophthalmology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Ophthalmology"),
         ("Orthopedics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Orthopedics"),
         ("Otolaryngology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Otolaryngology"),
         ("Pain Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Pain_Medicine"),
         ("Palliative Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Palliative_Medicine"),
         ("Pathology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Pathology"),
         ("Pediatrics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Pediatrics"),
         ("Pharmacology and Therapeutics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Pharmacology_and_Therapeutics"),
         ("Primary Care Research","http://connect.medrxiv.org/medrxiv_xml.php?subject=Primary_Care_Research"),
         ("Psychiatry and Clinical Psychology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Psychiatry_and_Clinical_Psychology"),
         ("Public and Global Health","http://connect.medrxiv.org/medrxiv_xml.php?subject=Public_and_Global_Health"),
         ("Radiology and Imaging","http://connect.medrxiv.org/medrxiv_xml.php?subject=Radiology_and_Imaging"),
         ("Rehabilitation Medicine and Physical Therapy","http://connect.medrxiv.org/medrxiv_xml.php?subject=Rehabilitation_Medicine_and_Physical_Therapy"),
         ("Respiratory Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Respiratory_Medicine"),
         ("Rheumatology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Rheumatology"),
         ("Sexual and Reproductive Health","http://connect.medrxiv.org/medrxiv_xml.php?subject=Sexual_and_Reproductive_Health"),
         ("Sports Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Sports_Medicine"),
         ("Surgery","http://connect.medrxiv.org/medrxiv_xml.php?subject=Surgery"),
         ("Toxicology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Toxicology"),
         ("Transplantation","http://connect.medrxiv.org/medrxiv_xml.php?subject=Transplantation"),
        (
            "Urology",
            "http://connect.medrxiv.org/medrxiv_xml.php?subject=Urology",
        ),
    ]
    .iter()
    .cloned()
    .collect();
    arxiv_sections_links
}
