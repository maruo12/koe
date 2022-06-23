use crate::voicevox::{GenerateQueryFromPresetParams, SynthesisParams, VoicevoxClient};
use anyhow::{anyhow, Result};
use koe_audio::EncodedAudio;

pub async fn make_speech(client: &VoicevoxClient, option: SpeechRequest) -> Result<EncodedAudio> {
    let preset_list = client.get_presets().await?;
    let preset = preset_list
        .get(&option.preset_id.0)
        .ok_or_else(|| anyhow!("Preset {} is not available", option.preset_id.0))?;

    let query = client
        .generate_query_from_preset(GenerateQueryFromPresetParams {
            preset_id: preset.id,
            text: option.text,
        })
        .await?;

    let audio = client
        .synthesis(SynthesisParams {
            style_id: preset.style_id,
            query,
        })
        .await?;

    Ok(audio)
}

pub async fn list_preset_ids(client: &VoicevoxClient) -> Result<Vec<PresetId>> {
    let preset_list = client.get_presets().await?;
    let ids = preset_list.into_keys().map(PresetId).collect();
    Ok(ids)
}

#[derive(Debug, Clone)]
pub struct SpeechRequest {
    pub text: String,
    pub preset_id: PresetId,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PresetId(pub i64);

impl From<i64> for PresetId {
    fn from(x: i64) -> Self {
        Self(x)
    }
}

impl From<&i64> for PresetId {
    fn from(x: &i64) -> Self {
        Self(*x)
    }
}

impl From<PresetId> for i64 {
    fn from(x: PresetId) -> Self {
        x.0
    }
}

impl From<&PresetId> for i64 {
    fn from(x: &PresetId) -> Self {
        x.0
    }
}
