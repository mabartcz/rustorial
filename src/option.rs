#[allow(dead_code)]
enum Exercise {
    Pushup,
    Squat,
    Plank,
    Flexion
}


fn create_plan() -> Option<Exercise> {
    let exer = Exercise::Squat;
    return Some(exer)
    // return Option::None
}

pub fn test_option() {
    let todo = create_plan();

    let result = match todo {
        Some(Exercise::Squat) => "Squat",
        Option::None => "None",
        _ => "ERROR"
    };

    println!("{result}");
}