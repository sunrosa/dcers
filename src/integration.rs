// Integration tests

use super::*;

#[test]
fn print_message_count_by_user() {
    let json: model::ExportedJson = json::read_export(
        "test_json/Direct Messages - Group - 1600s need not apply [1032681951666122772].json",
    )
    .unwrap();
    let users = stats::message_count_by_user(&json.messages);

    for user in users {
        println!("{}: {}", user.1 .0.name, user.1 .1);
    }
}

#[test]
fn print_all_users() {
    let json: model::ExportedJson = json::read_export(
        "test_json/Direct Messages - Group - 1600s need not apply [1032681951666122772].json",
    )
    .unwrap();
    let users = stats::all_users(&json.messages);
    for user in users {
        println!("{:?}", user);
    }
}

#[test]
fn print_word_count() {
    let json: model::ExportedJson = json::read_export(
        "test_json/Direct Messages - Group - 1600s need not apply [1032681951666122772].json",
    )
    .unwrap();
    let mut words: Vec<_> = stats::top_words(&json.messages, false, true)
        .into_iter()
        .collect();
    words.sort_by(|a, b| a.1.cmp(&b.1));
    for word in words {
        println!("{:?}", word);
    }
}

#[test]
fn print_total_word_count() {
    let json: model::ExportedJson = json::read_export(
        "test_json/Direct Messages - Group - 1600s need not apply [1032681951666122772].json",
    )
    .unwrap();
    println!("{:?}", stats::word_count(&json.messages));
}
