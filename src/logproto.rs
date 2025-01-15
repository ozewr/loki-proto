#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushRequest {
    #[prost(message, repeated, tag="1")]
    pub streams: ::prost::alloc::vec::Vec<StreamAdapter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAdapter {
    #[prost(string, tag="1")]
    pub labels: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub entries: ::prost::alloc::vec::Vec<EntryAdapter>,
    /// hash contains the original hash of the stream.
    #[prost(uint64, tag="3")]
    pub hash: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelPairAdapter {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryAdapter {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="2")]
    pub line: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub structured_metadata: ::prost::alloc::vec::Vec<LabelPairAdapter>,
    /// This field shouldn't be used by clients to push data to Loki.
    /// It is only used by Loki to return parsed log lines in query responses.
    /// TODO: Remove this field from the write path Proto.
    #[prost(message, repeated, tag="4")]
    pub parsed: ::prost::alloc::vec::Vec<LabelPairAdapter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelToValuesResponse {
    #[prost(map="string, message", tag="1")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, UniqueLabelValues>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniqueLabelValues {
    #[prost(string, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamRatesRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamRatesResponse {
    #[prost(message, repeated, tag="1")]
    pub stream_rates: ::prost::alloc::vec::Vec<StreamRate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamRate {
    #[prost(uint64, tag="1")]
    pub stream_hash: u64,
    #[prost(uint64, tag="2")]
    pub stream_hash_no_shard: u64,
    /// rate in plain bytes.
    #[prost(int64, tag="3")]
    pub rate: i64,
    #[prost(string, tag="4")]
    pub tenant: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub pushes: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[deprecated]
    #[prost(string, tag="1")]
    pub selector: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    #[prost(message, optional, tag="3")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration="Direction", tag="5")]
    pub direction: i32,
    #[prost(string, repeated, tag="7")]
    pub shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="8")]
    pub deletes: ::prost::alloc::vec::Vec<Delete>,
    #[prost(message, optional, tag="9")]
    pub plan: ::core::option::Option<Plan>,
    /// If populated, these represent the chunk references that the querier should
    /// use to fetch the data, plus any other chunks reported by ingesters.
    #[prost(message, optional, tag="10")]
    pub store_chunks: ::core::option::Option<ChunkRefGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleQueryRequest {
    /// mark as reserved once we've fully migrated to plan.
    #[deprecated]
    #[prost(string, tag="1")]
    pub selector: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, repeated, tag="4")]
    pub shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="5")]
    pub deletes: ::prost::alloc::vec::Vec<Delete>,
    #[prost(message, optional, tag="6")]
    pub plan: ::core::option::Option<Plan>,
    /// If populated, these represent the chunk references that the querier should
    /// use to fetch the data, plus any other chunks reported by ingesters.
    #[prost(message, optional, tag="10")]
    pub store_chunks: ::core::option::Option<ChunkRefGroup>,
}
/// TODO(owen-d): fix. This will break rollouts as soon as the internal repr is
/// changed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    #[prost(bytes="vec", tag="1")]
    pub raw: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delete {
    #[prost(string, tag="1")]
    pub selector: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub start: i64,
    #[prost(int64, tag="3")]
    pub end: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(message, repeated, tag="1")]
    pub streams: ::prost::alloc::vec::Vec<StreamAdapter>,
    #[prost(message, optional, tag="2")]
    pub stats: ::core::option::Option<super::stats::Ingester>,
    #[prost(string, repeated, tag="3")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleQueryResponse {
    #[prost(message, repeated, tag="1")]
    pub series: ::prost::alloc::vec::Vec<Series>,
    #[prost(message, optional, tag="2")]
    pub stats: ::core::option::Option<super::stats::Ingester>,
    #[prost(string, repeated, tag="3")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// True to fetch label values, false for fetch labels names.
    #[prost(bool, tag="2")]
    pub values: bool,
    #[prost(message, optional, tag="3")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    /// Naming this query instead of match because this should be
    #[prost(string, tag="5")]
    pub query: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelResponse {
    #[prost(string, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sample {
    #[prost(int64, tag="1")]
    pub timestamp: i64,
    #[prost(double, tag="2")]
    pub value: f64,
    #[prost(uint64, tag="3")]
    pub hash: u64,
}
/// LegacySample exists for backwards compatibility reasons and is deprecated. Do
/// not use.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacySample {
    #[prost(double, tag="1")]
    pub value: f64,
    #[prost(int64, tag="2")]
    pub timestamp_ms: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Series {
    #[prost(string, tag="1")]
    pub labels: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub samples: ::prost::alloc::vec::Vec<Sample>,
    #[prost(uint64, tag="3")]
    pub stream_hash: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailRequest {
    #[deprecated]
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub delay_for: u32,
    #[prost(uint32, tag="4")]
    pub limit: u32,
    #[prost(message, optional, tag="5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub plan: ::core::option::Option<Plan>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailResponse {
    #[prost(message, optional, tag="1")]
    pub stream: ::core::option::Option<StreamAdapter>,
    #[prost(message, repeated, tag="2")]
    pub dropped_streams: ::prost::alloc::vec::Vec<DroppedStream>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeriesRequest {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, repeated, tag="3")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeriesResponse {
    #[prost(message, repeated, tag="1")]
    pub series: ::prost::alloc::vec::Vec<SeriesIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeriesIdentifier {
    #[prost(message, repeated, tag="1")]
    pub labels: ::prost::alloc::vec::Vec<series_identifier::LabelsEntry>,
}
/// Nested message and enum types in `SeriesIdentifier`.
pub mod series_identifier {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LabelsEntry {
        #[prost(string, tag="1")]
        pub key: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub value: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DroppedStream {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub labels: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelPair {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// LegacyLabelPair exists for backwards compatibility reasons and is deprecated.
/// Do not use. Use LabelPair instead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyLabelPair {
    #[prost(bytes="vec", tag="1")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailersCountRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailersCountResponse {
    #[prost(uint32, tag="1")]
    pub count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunkIDsRequest {
    #[prost(string, tag="1")]
    pub matchers: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunkIDsResponse {
    #[prost(string, repeated, tag="1")]
    pub chunk_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ChunkRef contains the metadata to reference a Chunk.
/// It is embedded by the Chunk type itself and used to generate the Chunk
/// checksum. So it is imported to take care of the JSON representation of the
/// resulting Go struct.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkRef {
    #[prost(uint64, tag="1")]
    pub fingerprint: u64,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub from: i64,
    #[prost(int64, tag="4")]
    pub through: i64,
    /// The checksum is not written to the external storage. We use crc32,
    /// Castagnoli table. See <http://www.evanjones.ca/crc32c.html.>
    #[prost(uint32, tag="5")]
    pub checksum: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkRefGroup {
    #[prost(message, repeated, tag="1")]
    pub refs: ::prost::alloc::vec::Vec<ChunkRef>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelValuesForMetricNameRequest {
    #[prost(string, tag="1")]
    pub metric_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label_name: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub from: i64,
    #[prost(int64, tag="4")]
    pub through: i64,
    #[prost(string, tag="5")]
    pub matchers: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelNamesForMetricNameRequest {
    #[prost(string, tag="1")]
    pub metric_name: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub from: i64,
    #[prost(int64, tag="3")]
    pub through: i64,
}
/// TODO(owen-d): fix. This will break rollouts as soon as the internal repr is
/// changed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineFilter {
    #[prost(bytes="vec", tag="1")]
    pub raw: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunkRefRequest {
    #[prost(int64, tag="1")]
    pub from: i64,
    #[prost(int64, tag="2")]
    pub through: i64,
    #[prost(string, tag="3")]
    pub matchers: ::prost::alloc::string::String,
    /// TODO(salvacorts): Delete this field once the weekly release is done.
    #[prost(message, repeated, tag="4")]
    pub filters: ::prost::alloc::vec::Vec<LineFilter>,
    #[prost(message, optional, tag="5")]
    pub plan: ::core::option::Option<Plan>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunkRefResponse {
    #[prost(message, repeated, tag="1")]
    pub refs: ::prost::alloc::vec::Vec<ChunkRef>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSeriesRequest {
    #[prost(int64, tag="1")]
    pub from: i64,
    #[prost(int64, tag="2")]
    pub through: i64,
    #[prost(string, tag="3")]
    pub matchers: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSeriesResponse {
    #[prost(message, repeated, tag="1")]
    pub series: ::prost::alloc::vec::Vec<IndexSeries>,
}
/// Series calls to the TSDB Index
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexSeries {
    #[prost(message, repeated, tag="1")]
    pub labels: ::prost::alloc::vec::Vec<LabelPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIndexResponse {
    #[prost(string, tag="1")]
    pub query_key: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(bytes="vec", tag="1")]
    pub range_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIndexRequest {
    #[prost(message, repeated, tag="1")]
    pub queries: ::prost::alloc::vec::Vec<IndexQuery>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexQuery {
    #[prost(string, tag="1")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub hash_value: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub range_value_prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub range_value_start: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub value_equal: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexStatsRequest {
    #[prost(int64, tag="1")]
    pub from: i64,
    #[prost(int64, tag="2")]
    pub through: i64,
    /// TODO(owen-d): add shards to grpc calls so we don't have
    /// to extract via labels
    #[prost(string, tag="3")]
    pub matchers: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexStatsResponse {
    #[prost(uint64, tag="1")]
    pub streams: u64,
    #[prost(uint64, tag="2")]
    pub chunks: u64,
    #[prost(uint64, tag="3")]
    pub bytes: u64,
    #[prost(uint64, tag="4")]
    pub entries: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeRequest {
    #[prost(int64, tag="1")]
    pub from: i64,
    #[prost(int64, tag="2")]
    pub through: i64,
    #[prost(string, tag="3")]
    pub matchers: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub limit: i32,
    #[prost(int64, tag="5")]
    pub step: i64,
    #[prost(string, repeated, tag="6")]
    pub target_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="7")]
    pub aggregate_by: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeResponse {
    #[prost(message, repeated, tag="1")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    #[prost(int32, tag="2")]
    pub limit: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub volume: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectedFieldsRequest {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    /// Naming this query instead of match because this should be
    #[prost(string, tag="3")]
    pub query: ::prost::alloc::string::String,
    /// with queryrangebase.Request interface
    #[prost(uint32, tag="4")]
    pub line_limit: u32,
    #[prost(uint32, tag="5")]
    pub field_limit: u32,
    #[prost(int64, tag="6")]
    pub step: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectedFieldsResponse {
    #[prost(message, repeated, tag="1")]
    pub fields: ::prost::alloc::vec::Vec<DetectedField>,
    #[prost(uint32, tag="2")]
    pub field_limit: u32,
}
/// TODO: make the detected field include the serialized sketch
/// we only want cardinality in the JSON response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectedField {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub cardinality: u64,
    #[prost(string, repeated, tag="4")]
    pub parsers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="5")]
    pub sketch: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectedLabelsRequest {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub query: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectedLabelsResponse {
    #[prost(message, repeated, tag="1")]
    pub detected_labels: ::prost::alloc::vec::Vec<DetectedLabel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectedLabel {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub cardinality: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Direction {
    Forward = 0,
    Backward = 1,
}
