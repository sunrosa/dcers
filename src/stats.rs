use super::*;

use std::collections::HashMap;

/// Get the number of messages sent per user.
/// # Returns
/// A hashmap of Discord user IDs and a tuple of the user's data and the amount of messages sent by that user.
pub fn message_count_by_user(
    messages: &Vec<model::Message>,
) -> HashMap<&model::Id, (&model::User, u32)> {
    let mut users: HashMap<&model::Id, (&model::User, u32)> = Default::default();
    for message in messages {
        users.insert(
            &message.author.id,
            match users.get(&message.author.id) {
                Some(u) => (&message.author, u.1 + 1),
                None => (&message.author, 1),
            },
        );
    }

    return users;
}

/// Create a vector containing a unique list of all users that have sent messages contained in the list of messages.
pub fn all_users(messages: &Vec<model::Message>) -> Vec<&model::User> {
    let mut users: HashMap<&model::Id, &model::User> = Default::default();
    for message in messages {
        users.insert(&message.author.id, &message.author);
    }
    return users.values().cloned().collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_message_count_by_user() {
        let json = json::read_export(
            "VRChess club - Server Info - scam_awareness [930732429113696296].json",
        )
        .unwrap();
        let count = message_count_by_user(&json.messages);
        assert_eq!(
            count,
            HashMap::from([
                (
                    &"204366690446737419".to_owned(),
                    (
                        &model::User {
                            id: "204366690446737419".to_owned(),
                            name: "ayachii".to_owned(),
                            discriminator: "0001".to_owned(),
                            nickname: "ayachii".to_owned(),
                            is_bot: false
                        },
                        1
                    )
                ),
                (
                    &"234134781552492545".to_owned(),
                    (
                        &model::User {
                            id: "234134781552492545".to_owned(),
                            name: "Astrea".to_owned(),
                            discriminator: "1000".to_owned(),
                            nickname: "Astrea".to_owned(),
                            is_bot: false
                        },
                        4
                    )
                ),
                (
                    &"544005059353247754".to_owned(),
                    (
                        &model::User {
                            id: "544005059353247754".to_owned(),
                            name: "Deimos1289".to_owned(),
                            discriminator: "9434".to_owned(),
                            nickname: "Deimos".to_owned(),
                            is_bot: false
                        },
                        3
                    )
                ),
            ])
        )
    }
}
