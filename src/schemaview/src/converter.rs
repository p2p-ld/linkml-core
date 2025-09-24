#[cfg(feature = "curies")]
mod curies_backend {
    pub use curies::error::CuriesError as ConverterError;
    pub use curies::{Converter, Record};
}

#[cfg(not(feature = "curies"))]
mod minimal_backend {
    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;

    use thiserror::Error;

    /// Record describing how to expand/compress a prefix.
    #[derive(Debug, Clone)]
    pub struct Record {
        pub prefix: String,
        pub uri_prefix: String,
        pub prefix_synonyms: HashSet<String>,
        pub uri_prefix_synonyms: HashSet<String>,
        pub pattern: Option<String>,
    }

    impl Record {
        /// Construct a new record without synonyms.
        pub fn new(prefix: &str, uri_prefix: &str) -> Self {
            Self {
                prefix: prefix.to_string(),
                uri_prefix: uri_prefix.to_string(),
                prefix_synonyms: HashSet::new(),
                uri_prefix_synonyms: HashSet::new(),
                pattern: None,
            }
        }

        fn all_prefixes(&self) -> impl Iterator<Item = &String> {
            std::iter::once(&self.prefix).chain(self.prefix_synonyms.iter())
        }

        fn all_uri_prefixes(&self) -> impl Iterator<Item = &String> {
            std::iter::once(&self.uri_prefix).chain(self.uri_prefix_synonyms.iter())
        }
    }

    /// Error raised while converting identifiers.
    #[derive(Debug, Error, Clone, PartialEq, Eq)]
    pub enum ConverterError {
        #[error("prefix or URI not found: {0}")]
        NotFound(String),
        #[error("invalid CURIE: {0}")]
        InvalidCurie(String),
        #[error("duplicate record for {0}")]
        DuplicateRecord(String),
    }

    /// Minimal CURIE/URI converter used by LinkML tooling.
    ///
    /// This is intentionally lightweight and dependency free so the workspace stays
    /// buildable in WASM and SBOM-friendly contexts. If future needs require
    /// advanced features that would force pulling in additional crates (regex,
    /// async fetch, etc.), prefer enabling the `curies` feature to use the upstream
    /// implementation instead of expanding this module.
    #[derive(Debug, Clone)]
    pub struct Converter {
        delimiter: String,
        prefix_map: HashMap<String, Arc<Record>>,
        uri_map: HashMap<String, Arc<Record>>,
        records: Vec<Arc<Record>>,
    }

    impl Converter {
        /// Create a new converter with a custom delimiter (defaults to `:`).
        pub fn new(delimiter: &str) -> Self {
            Self {
                delimiter: delimiter.to_string(),
                prefix_map: HashMap::new(),
                uri_map: HashMap::new(),
                records: Vec::new(),
            }
        }

        /// Add a complete record including synonyms.
        pub fn add_record(&mut self, record: Record) -> Result<(), ConverterError> {
            let prefixes: Vec<String> = record.all_prefixes().map(|p| p.to_string()).collect();
            let uri_prefixes: Vec<String> =
                record.all_uri_prefixes().map(|p| p.to_string()).collect();

            for prefix in &prefixes {
                if self.prefix_map.contains_key(prefix) {
                    return Err(ConverterError::DuplicateRecord(prefix.clone()));
                }
            }
            for uri_prefix in &uri_prefixes {
                if self.uri_map.contains_key(uri_prefix) {
                    return Err(ConverterError::DuplicateRecord(uri_prefix.clone()));
                }
            }

            let record = Arc::new(record);
            for prefix in prefixes {
                self.prefix_map.insert(prefix, Arc::clone(&record));
            }
            for uri_prefix in uri_prefixes {
                self.uri_map.insert(uri_prefix, Arc::clone(&record));
            }
            self.records.push(record);
            Ok(())
        }

        /// Convenience helper to add a prefix that has no synonyms.
        pub fn add_prefix(&mut self, prefix: &str, uri_prefix: &str) -> Result<(), ConverterError> {
            self.add_record(Record::new(prefix, uri_prefix))
        }

        /// Locate a record by prefix (including synonyms).
        pub fn find_by_prefix(&self, prefix: &str) -> Result<&Arc<Record>, ConverterError> {
            self.prefix_map
                .get(prefix)
                .ok_or_else(|| ConverterError::NotFound(prefix.to_string()))
        }

        /// Expand a CURIE into a full URI.
        pub fn expand(&self, curie: &str) -> Result<String, ConverterError> {
            let parts: Vec<&str> = curie.splitn(2, &self.delimiter).collect();
            if parts.len() != 2 {
                return Err(ConverterError::InvalidCurie(curie.to_string()));
            }
            let record = self.find_by_prefix(parts[0])?;
            Ok(format!("{}{}", record.uri_prefix, parts[1]))
        }

        /// Compress a URI to a CURIE using the longest matching URI prefix.
        pub fn compress(&self, uri: &str) -> Result<String, ConverterError> {
            let mut best: Option<(Arc<Record>, usize)> = None;
            for record in &self.records {
                let mut match_prefix: Option<usize> = None;
                for candidate in record.all_uri_prefixes() {
                    if uri.starts_with(candidate) {
                        let len = candidate.len();
                        if match_prefix.is_none_or(|current| len > current) {
                            match_prefix = Some(len);
                        }
                    }
                }

                if let Some(prefix_len) = match_prefix {
                    if best
                        .as_ref()
                        .is_none_or(|(_, best_len)| prefix_len > *best_len)
                    {
                        best = Some((Arc::clone(record), prefix_len));
                    }
                }
            }

            let (record, prefix_len) =
                best.ok_or_else(|| ConverterError::NotFound(uri.to_string()))?;
            let suffix = &uri[prefix_len..];
            Ok(format!("{}{}{}", record.prefix, self.delimiter, suffix))
        }
    }

    impl Default for Converter {
        fn default() -> Self {
            Self::new(":")
        }
    }
}

#[cfg(feature = "curies")]
/// Re-export the upstream implementation when the `curies` feature is enabled.
pub use curies_backend::{Converter, ConverterError, Record};

#[cfg(not(feature = "curies"))]
pub use minimal_backend::{Converter, ConverterError, Record};
