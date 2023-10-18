const LAB_WEIGHT: f64 = 0.4;
const EXAM_WEIGHT: f64 = 0.6;


fn main() {
    println!("Hello, world!");

    println!("Calculate");
    
    let lab_score = LAB_WEIGHT*58.0;
    let exam_score = EXAM_WEIGHT*43.0;
    let score = lab_score+exam_score;
    let grade = match score {
        // inclusive on both ends สร้างช่วงที่รวมทั้งสองขอบ
        80.0..=100.0 => "A",
        75.0..=79.0 => "B+",
        70.0..=74.0 => "B",
        65.0..=69.0 => "C+",
        60.0..=64.0 => "C",
        55.0..=59.0 => "D+",
        50.0..=54.0 => "D",
        _ => "F",
    };

    println!("Lab score : {:.2}",lab_score);
    println!("Exam score : {:.2}",exam_score);
    println!("Total score : {:.2}", score);
    println!("Grade : {:.2}",grade);
}

// 100 *0.4=40
// 100 *0.6=60
//          100/1.0
    