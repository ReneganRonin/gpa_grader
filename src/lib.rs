use std::process;

pub fn collect_subs(args: &[String]) -> Vec<Subject> {
    let mut all_subjects: Vec<Subject> = Vec::new();
    for subs in 0..args[1..].len() {
        if subs % 3 == 0 {
            if subs == 0 {
                let subject = Subject::new(&args[..=subs + 3]).unwrap_or_else(|err| {
                    eprintln!("Problem parsing arguments: {}", err);
                    process::exit(1);
                });

                all_subjects.push(subject);
            } else {
                let subject = Subject::new(&args[subs..=subs + 3]).unwrap_or_else(|err| {
                    eprintln!("Problem parsing arguments: {}", err);
                    process::exit(1);
                });

                all_subjects.push(subject);
            };
        };
    }
    all_subjects
}

pub fn run(all_subjects: Vec<Subject>) {
    let cumulative_score: f64 = cumulative_score(&all_subjects);
    let total_credits: f64 = total_credits(&all_subjects);
    for subject in all_subjects.iter() {
        let name = &subject.name;
        let credit = subject.credits;
        let grade = subject.grade;
        println!(
            "Subject Code: {}, Credit: {}, Grade: {:.2}",
            name, credit, grade
        );
    }
    println!("GPA: {:.4}", gwa(total_credits, cumulative_score));
}

pub fn names_of_subjects(subjects: &[Subject]) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for name in subjects.iter() {
        names.push(name.name.clone())
    }
    names
}

pub fn total_credits(subjects: &[Subject]) -> f64 {
    let mut total_credits: f64 = 0.0;
    for credit in subjects.iter() {
        total_credits += credit.credits as f64;
    }
    total_credits
}

pub fn cumulative_score(subjects: &[Subject]) -> f64 {
    let mut cumulative_score: f64 = 0.0;
    for scores in subjects.iter() {
        cumulative_score += scores.product;
    }
    cumulative_score
}

pub fn gwa(total_credits: f64, cumulative_score: f64) -> f64 {
    cumulative_score / total_credits
}

pub struct Subject {
    pub name: String,
    pub credits: i64,
    pub grade: f64,
    pub product: f64,
}

impl Subject {
    pub fn new(args: &[String]) -> Result<Subject, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let name = args[1].clone();
        let credits = args[2].clone().parse::<i64>().unwrap_or_else(|err| {
            eprintln!("Credits should be numeric, got {}", err);
            process::exit(1);
        });
        let grade = args[3].clone().parse::<f64>().unwrap_or_else(|err| {
            eprintln!("Grade should be numeric, got {}", err);
            process::exit(1);
        });

        let product = credits as f64 * grade;
        Ok(Subject {
            name,
            credits,
            grade,
            product,
        })
    }
}

// // Tests
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn equal_total_credit() {
//         let test_args: Vec<String> = vec![
//             "dummyfirsttargetname".to_string(),
//             "BIO107".to_string(),
//             "5".to_string(),
//             "2.00".to_string(),
//             "BIO108".to_string(),
//             "5".to_string(),
//             "1.50".to_string(),
//             "FIL101".to_string(),
//             "3".to_string(),
//             "1.50".to_string(),
//             "GEC109".to_string(),
//             "3".to_string(),
//             "1.00".to_string(),
//             "HIS003".to_string(),
//             "3".to_string(),
//             "1.00".to_string(),
//         ];
//         let mut _all_subjects: Vec<Subject> = collect_subs(&test_args);
//         let total_credits: f64 = total_credits(&_all_subjects);

//         assert_eq!(12.00, total_credits); // remembered that it is dumb to compare primitive floating types
//         I might use a crate for comparing approximation of the values but I realized for a simple program like this?
//         that would just increase the complexity of a simple project lol
//     }
// }
