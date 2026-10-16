use serde::{Deserialize, Serialize};

/// Account bucket listing, returned by GET Service.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBucketsResult {
    /// Accessible buckets.
    pub buckets: Buckets,
}
/// XML wrapper for the bucket list.
#[derive(Debug, Deserialize)]
pub struct Buckets {
    /// Bucket entries; empty accounts have an empty list.
    #[serde(rename = "Bucket", default)]
    pub items: Vec<Bucket>,
}
/// One bucket's location and creation metadata.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    /// Full bucket name including APPID.
    pub name: String,
    /// COS region.
    pub location: String,
    /// Server-provided creation timestamp.
    pub creation_date: String,
}
/// Parameters for GET Bucket; pagination is explicit, not automatic.
#[derive(Debug, Default)]
pub struct ListObjectsOptions {
    /// Filter keys by this literal prefix.
    pub prefix: Option<String>,
    /// Continue after a previous key or `NextMarker`.
    pub marker: Option<String>,
    /// Group keys using a delimiter, commonly '/'.
    pub delimiter: Option<String>,
    /// Maximum keys in one page, from 1 through 1000.
    pub max_keys: Option<u16>,
}
/// One page of objects and common prefixes.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectsResult {
    /// Full bucket name.
    pub name: String,
    /// Whether another page may exist.
    pub is_truncated: bool,
    /// Explicit marker for the next call, if supplied by COS.
    pub next_marker: Option<String>,
    /// Objects on this page.
    #[serde(rename = "Contents", default)]
    pub objects: Vec<Object>,
    /// Delimiter-grouped prefixes.
    #[serde(default)]
    pub common_prefixes: Vec<CommonPrefix>,
}
/// Metadata for one listed object.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Object {
    /// Literal object key, not URL-encoded by this client.
    pub key: String,
    /// Server modification timestamp.
    pub last_modified: String,
    /// Opaque entity tag; not necessarily an MD5 checksum.
    #[serde(rename = "ETag")]
    pub etag: String,
    /// Object size in bytes.
    pub size: u64,
    /// Server storage class, if present.
    pub storage_class: Option<String>,
}
/// One common prefix in an object listing.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonPrefix {
    /// Grouped prefix.
    pub prefix: String,
}
/// Copy-object result.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CopyObjectResult {
    /// Destination entity tag.
    #[serde(rename = "ETag")]
    pub etag: String,
    /// Modification timestamp.
    pub last_modified: String,
}
/// Initiated multipart upload. Persist `UploadId` only under application control.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultipartUpload {
    /// Target bucket.
    pub bucket: String,
    /// Target object key.
    pub key: String,
    /// Opaque upload identifier.
    #[serde(rename = "UploadId")]
    pub upload_id: String,
}
/// A successfully uploaded part, passed back in ascending order for completion.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompletedPart {
    /// Part number, in 1..=10000.
    pub part_number: u16,
    /// Opaque server entity tag, including quotes when supplied by COS.
    #[serde(rename = "ETag")]
    pub etag: String,
}
/// One page of uploaded parts.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListPartsResult {
    /// Target bucket.
    pub bucket: String,
    /// Target object key.
    pub key: String,
    /// Upload identifier.
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    /// Whether another page exists.
    pub is_truncated: bool,
    /// Marker for explicit pagination.
    pub next_part_number_marker: Option<u16>,
    /// Uploaded parts on this page.
    #[serde(rename = "Part", default)]
    pub parts: Vec<CompletedPart>,
}
/// Completed multipart object metadata.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompleteMultipartResult {
    /// Result object location, if supplied.
    pub location: Option<String>,
    /// Destination bucket.
    pub bucket: String,
    /// Destination key.
    pub key: String,
    /// Final opaque entity tag.
    #[serde(rename = "ETag")]
    pub etag: String,
}
