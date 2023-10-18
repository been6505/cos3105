fn main() {
    println!("Hello, world!");

    println!("Calculate Grade");
    
    let lab_score = 0.4*58.0;
    let exam_score = 0.6*43.0;
    let score = lab_score+exam_score;
    let grade = match score {
        80.0..=100.0 => "A",
        75.0..=79.0 => "B+",
        70.0..=74.0 => "B",
        65.0..=69.0 => "C+",
        60.0..=64.0 => "C",
        55.0..=59.0 => "D+",
        50.0..=54.0 => "D",
        _ => "F",
    };

    println!("Lab score : {}",lab_score);
    println!("Exam score : {}",exam_score);
    println!("Total score : {}", score);
    println!("Grade : {}",grade);
}