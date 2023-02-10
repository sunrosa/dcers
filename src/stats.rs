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

/// Create a Vector containing a unique list of all users that have sent messages contained in the list of messages.
pub fn all_users(messages: &Vec<model::Message>) -> Vec<&model::User> {
    let mut users: HashMap<&model::Id, &model::User> = Default::default();
    for message in messages {
        users.insert(&message.author.id, &message.author);
    }
    return users.values().cloned().collect();
}

/// Create a HashMap containing a list of words and the number of times they've each been used.
/// * `messages` - The Discord messages to be processed.
/// * `case_sensitive` - Convert the words to lowercase before processing them.
/// * `restrict_to_alphanumeric` - Delete all non-alphanumeric characters before processing them.
pub fn top_words(
    messages: &Vec<model::Message>,
    case_sensitive: bool,
    restrict_to_alphanumeric: bool,
) -> HashMap<String, u32> {
    let mut words: HashMap<String, u32> = Default::default();

    for message in messages {
        for word in message.content.split(' ') {
            let mut word_modified: String = word.to_string();
            if restrict_to_alphanumeric {
                word_modified = word_modified
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect();
            }
            if !case_sensitive {
                word_modified = word_modified.to_lowercase();
            }
            if word_modified == "" {
                continue;
            }
            words.insert(
                word_modified.to_owned(),
                match words.get(&word_modified) {
                    Some(w) => w + 1,
                    None => 1,
                },
            );
        }
    }

    return words;
}

/// Total word count of messages, separated by spaces.
pub fn word_count(messages: &Vec<model::Message>) -> u32 {
    let mut word_count: u32 = 0;

    for message in messages {
        word_count += message.content.split(' ').count() as u32;
    }

    return word_count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_message_count_by_user() {
        let json = json::read_export(
            "test_json/VRChess club - Server Info - scam_awareness [930732429113696296].json",
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
