/// Typed, metadata-only usage restrictions attached to a dataset.
///
/// These restrictions are intentionally distinct from `AccessPolicy`:
/// the data may remain publicly downloadable while still carrying source-level
/// publication or reuse constraints that clients should surface.
#[non_exhaustive]
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
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DataUseRestrictionKind {
    /// The source allows public download, but biological discovery/publication
    /// use remains embargoed until the source project publishes.
    SourcePublicationEmbargo,
}

/// Structured metadata describing a dataset-specific usage restriction.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DataUseRestrictionSpec {
    /// Machine-readable restriction category.
    pub kind: DataUseRestrictionKind,
    /// Human-readable summary suitable for metadata display.
    pub summary: String,
    /// Optional canonical source describing the restriction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
}
