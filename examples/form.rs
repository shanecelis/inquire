use survey_rs::{
    ask::AskMany, confirm::ConfirmOptions, input::InputOptions, multiselect::MultiSelectOptions,
    select::SelectOptions,
};

extern crate survey_rs;

fn main() {
    let fruits = vec![
        "Banana",
        "Apple",
        "Strawberry",
        "Grapes",
        "Lemon",
        "Tangerine",
        "Watermelon",
        "Orange",
        "Pear",
        "Avocado",
        "Pineapple",
    ];

    let languages = vec![
        "C++",
        "Rust",
        "C",
        "Python",
        "Java",
        "TypeScript",
        "JavaScript",
        "Go",
    ];

    let questions = vec![
        InputOptions::new("Where do you work?")
            .with_help_message("Don't worry, this will not be sold to third-party advertisers.")
            .into(),
        SelectOptions::new("What's your favorite fruit?", &fruits)
            .unwrap()
            .into(),
        ConfirmOptions::new("Do you eat pizza?")
            .with_default(true)
            .into(),
        MultiSelectOptions::new("Which languages do you use at work?", &languages)
            .unwrap()
            .into(),
    ]
    .into_iter();

    let answers = questions.ask().unwrap();

    for (i, ans) in answers.iter().enumerate() {
        println!("Ans #{} is {}", i, ans);
    }
}
