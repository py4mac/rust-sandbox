pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 18);

    println!("{}, {}, {}", person.0, person.1, person.2);
}