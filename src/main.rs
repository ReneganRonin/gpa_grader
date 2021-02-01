use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut _all_subjects: Vec<Subject> = collect_subs(&args);
    run(_all_subjects);
}

fn collect_subs(args: &[String]) -> Vec<Subject> {
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

fn run(all_subjects: Vec<Subject>) {
    let mut cumulative_score: Vec<f64> = Vec::new();
    let mut total_credits: Vec<f64> = Vec::new();
    for subject in all_subjects.iter() {
        let name = &subject.name;
        let credits = subject.credits;
        let grade = subject.grade;
        total_credits.push(credits as f64);
        cumulative_score.push(subject.product);

        println!(
            "Subject Code: {}, Credit: {}, Grade: {:.2}",
            name, credits, grade
        );
    }
    println!("GWA: {:.4}", gwa(&total_credits, &cumulative_score));
}

fn gwa(total_credits: &[f64], cumulative_score: &[f64]) -> f64 {
    cumulative_score.iter().sum::<f64>() / total_credits.iter().sum::<f64>()
}

struct Subject {
    name: String,
    credits: i64,
    grade: f64,
    product: f64,
}

impl Subject {
    fn new(args: &[String]) -> Result<Subject, &'static str> {
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
