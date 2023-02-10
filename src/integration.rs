// Integration tests

use super::*;

#[test]
fn print_message_count_by_user() {
    let json: model::ExportedJson = json::read_export(
        "Direct Messages - Group - 1600s need not apply [1032681951666122772].json",
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
        "Direct Messages - Group - 1600s need not apply [1032681951666122772].json",
    )
    .unwrap();
    let users = stats::all_users(&json.messages);
    for user in users {
        println!("{:?}", user);
    }
}
