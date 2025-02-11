//! Fedora CoreOS metadata.

use serde_derive::Deserialize;

/// Templated URL for release index.
pub static RELEASES_JSON: &str =
    "https://builds.coreos.fedoraproject.org/prod/streams/${stream}/releases.json";

/// Templated URL for updates metadata.
pub static UPDATES_JSON: &str = "https://builds.coreos.fedoraproject.org/updates/${stream}.json";

pub static SCHEME: &str = "org.fedoraproject.coreos.scheme";

pub static AGE_INDEX: &str = "org.fedoraproject.coreos.releases.age_index";
pub static ARCH_PREFIX: &str = "org.fedoraproject.coreos.releases.arch";

pub static BARRIER: &str = "org.fedoraproject.coreos.updates.barrier";
pub static BARRIER_REASON: &str = "org.fedoraproject.coreos.updates.barrier_reason";
pub static DEADEND: &str = "org.fedoraproject.coreos.updates.deadend";
pub static DEADEND_REASON: &str = "org.fedoraproject.coreos.updates.deadend_reason";
pub static ROLLOUT: &str = "org.fedoraproject.coreos.updates.rollout";
pub static DURATION: &str = "org.fedoraproject.coreos.updates.duration_minutes";
pub static START_EPOCH: &str = "org.fedoraproject.coreos.updates.start_epoch";
pub static START_VALUE: &str = "org.fedoraproject.coreos.updates.start_value";

/// Fedora CoreOS release index.
#[derive(Clone, Debug, Deserialize)]
pub struct ReleasesJSON {
    pub releases: Vec<Release>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Release {
    pub commits: Vec<ReleaseCommit>,
    #[serde(rename = "oci-images")]
    pub oci_images: Option<Vec<ReleaseOciImage>>,
    pub version: String,
    pub metadata: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReleaseCommit {
    pub architecture: String,
    pub checksum: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReleaseOciImage {
    pub architecture: String,
    pub image: String,
    #[serde(rename = "digest-ref")]
    pub digest_ref: String,
}

/// Fedora CoreOS updates metadata
#[derive(Clone, Debug, Deserialize)]
pub struct UpdatesJSON {
    pub stream: String,
    pub releases: Vec<ReleaseUpdate>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReleaseUpdate {
    pub version: String,
    pub metadata: UpdateMetadata,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateMetadata {
    pub barrier: Option<UpdateBarrier>,
    pub deadend: Option<UpdateDeadend>,
    pub rollout: Option<UpdateRollout>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateBarrier {
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateDeadend {
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateRollout {
    pub start_epoch: Option<i64>,
    pub start_percentage: Option<f64>,
    pub duration_minutes: Option<u64>,
}
