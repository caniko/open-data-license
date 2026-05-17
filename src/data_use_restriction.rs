use utoipa::ToSchema;

/// Typed, metadata-only usage restrictions attached to a dataset.
///
/// These restrictions are intentionally distinct from `AccessPolicy`:
/// the data may remain publicly downloadable while still carrying source-level
/// publication or reuse constraints that clients should surface.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DataUseRestrictionKind {
    /// The source allows public download, but biological discovery/publication
    /// use remains embargoed until the source project publishes.
    SourcePublicationEmbargo,
}

/// Structured metadata describing a dataset-specific usage restriction.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct DataUseRestrictionSpec {
    pub kind: DataUseRestrictionKind,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
}
