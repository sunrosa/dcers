use super::*;

use std::collections::HashMap;

/// Get the number of messages sent per user.
/// # Returns
/// A hashmap of Discord user IDs and the amount of messages sent.
pub fn message_count_by_user(messages: &Vec<model::Message>) -> HashMap<model::Id, u32> {
    let mut users: HashMap<model::Id, u32> = Default::default();
    for message in messages {
        users.insert(
            message.author.id.clone(),
            match users.get(&message.author.id) {
                Some(u) => u + 1,
                None => 1,
            },
        );
    }

    return users;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn message_count_by_user() {
        let json = json::read_export(
            "VRChess club - Server Info - scam_awareness [930732429113696296].json",
        )
        .unwrap();
        let count = super::message_count_by_user(&json.messages);
        assert_eq!(
            count,
            HashMap::from([
                ("234134781552492545".to_owned(), 4),
                ("544005059353247754".to_owned(), 3),
                ("204366690446737419".to_owned(), 1)
            ])
        )
    }
}
