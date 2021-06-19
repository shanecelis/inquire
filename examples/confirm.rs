use survey_rs::{ask::Question, confirm::ConfirmOptions};

extern crate survey_rs;

fn main() {
    let ans = Question::Confirm(
        ConfirmOptions::new("Do you live in Brazil?")
            .with_default(false)
            .with_help_message("This data is stored for good reasons"),
    )
    .ask()
    .unwrap();

    println!("Final answer was {}", ans);
}
