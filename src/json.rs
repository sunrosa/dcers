use super::*;

/// Read a DiscordChatExporter exported JSON file from file to an ExportedJson.
/// * `path` - The file path to be read.
pub fn read_export(path: &str) -> anyhow::Result<model::ExportedJson> {
    let file = std::fs::File::open(path)?;
    let json: model::ExportedJson = serde_json::from_reader(file)?;
    Ok(json)
}
