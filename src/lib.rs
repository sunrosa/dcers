pub mod model;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Does the model deserialize without error? Tested on a medium-sized group DM.
    fn model_1() {
        let file = std::fs::File::open(
            "Direct Messages - Group - 1600s need not apply [1032681951666122772].json",
        )
        .expect("Could not open test file.");
        let json: model::ExportedJson =
            serde_json::from_reader(file).expect("Could not parse json.");
    }
}
