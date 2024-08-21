/// TODO list written in Rust
/// New <list>
/// Add <list> <item>
/// if selected list:
///     Add <item>
/// Done <list> <item>
/// Select <list> (focus on this list)
/// Fin (finished)

fn main() {
    // if there is more than one list,
    // is there a selected list?
    let is_selected: bool = false;
    // Ref selected list
    // since this can change we want to make sure
    // that the memory is assigned at RT.
    let mut selected: String = String::new();
    if selected.is_empty() {
        selected = "No List Selected!".to_string();
    }
    println!("List Selected: {}\nSelected List: {}", is_selected, selected);
}
