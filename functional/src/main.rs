use functional::simulated_expensive_calculation;
use functional::generate_workout;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // simulated_expensive_calculation(1);
    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z == x;
    assert!(equal_to_x(4));
}
