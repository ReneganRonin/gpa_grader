use gpa_grader::Subject;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut _all_subjects: Vec<Subject> = gpa_grader::collect_subs(&args);
    gpa_grader::run(_all_subjects);
}
