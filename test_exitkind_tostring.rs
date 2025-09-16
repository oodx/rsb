use rsb::com::ExitKind;

fn main() {
    println!("Testing ExitKind ToString implementation:");
    println!("Success: {}", ExitKind::Success.to_string());
    println!("Failure: {}", ExitKind::Failure.to_string());
    println!("SystemFailure: {}", ExitKind::SystemFailure.to_string());
    println!("LogicFailure: {}", ExitKind::LogicFailure.to_string());
    println!("UserFailure: {}", ExitKind::UserFailure.to_string());

    // Test that it works with is_true_val
    use rsb::com::is_true_val;
    println!("\nTesting with is_true_val:");
    println!("is_true_val(Success.to_string()): {}", is_true_val(&ExitKind::Success.to_string()));
    println!("is_true_val(Failure.to_string()): {}", is_true_val(&ExitKind::Failure.to_string()));
}