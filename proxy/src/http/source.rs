//! Endpoints and serialisation for source code browsing.

use librad::paths::Paths;
use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::coco;
use crate::identity;

/// Prefixed filters.
pub fn routes(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("source").and(
        blob_filter(Arc::<RwLock<Paths>>::clone(&paths))
            .or(branches_filter(Arc::<RwLock<Paths>>::clone(&paths)))
            .or(commit_filter(Arc::<RwLock<Paths>>::clone(&paths)))
            .or(commits_filter(Arc::<RwLock<Paths>>::clone(&paths)))
            .or(local_branches_filter())
            .or(revisions_filter(Arc::<RwLock<Paths>>::clone(&paths)))
            .or(tags_filter(Arc::<RwLock<Paths>>::clone(&paths)))
            .or(tree_filter(paths)),
    )
}

/// Combination of all source filters.
#[cfg(test)]
fn filters(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    blob_filter(Arc::<RwLock<Paths>>::clone(&paths))
        .or(branches_filter(Arc::<RwLock<Paths>>::clone(&paths)))
        .or(commit_filter(Arc::<RwLock<Paths>>::clone(&paths)))
        .or(commits_filter(Arc::<RwLock<Paths>>::clone(&paths)))
        .or(local_branches_filter())
        .or(revisions_filter(Arc::<RwLock<Paths>>::clone(&paths)))
        .or(tags_filter(Arc::<RwLock<Paths>>::clone(&paths)))
        .or(tree_filter(paths))
}

/// `GET /blob/<project_id>/<revision>/<path...>`
fn blob_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("blob")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(warp::filters::query::query::<BlobQuery>())
        .and(document::document(
            document::query("revision", document::string()).description("Git revision"),
        ))
        .and(document::document(
            document::query("path", document::string())
                .description("Location of the file in the repo tree"),
        ))
        .and(document::document(document::description("Fetch a Blob")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(coco::Blob::document()).mime("application/json"),
            )
            .description("Blob for path found"),
        ))
        .and_then(handler::blob)
}

/// `GET /branches/<project_id>`
fn branches_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("branches")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::document(document::description("List Branches")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(
                    document::array(coco::Branch::document()).description("List of branches"),
                )
                .mime("application/json"),
            )
            .description("List of branches"),
        ))
        .and_then(handler::branches)
}

/// `GET /commit/<project_id>/<sha1>`
fn commit_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("commit")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::param::<String>("sha1", "Git object id"))
        .and(document::document(document::description("Fetch a Commit")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(coco::Commit::document()).mime("application/json"),
            )
            .description("Commit for SHA1 found"),
        ))
        .and_then(handler::commit)
}

/// `GET /commits/<project_id>/<branch>`
fn commits_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("commits")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::param::<String>("branch", "Branch name"))
        .and(document::document(document::description(
            "Fetch Commits from a Branch",
        )))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(document::array(coco::Commit::document())).mime("application/json"),
            )
            .description("Branch found"),
        ))
        .and_then(handler::commits)
}

/// `GET /branches/<project_id>`
fn local_branches_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("local-branches")
        .and(warp::get())
        .and(document::tail(
            "path",
            "Location of the repository on the filesystem",
        ))
        .and(document::document(document::description(
            "List Branches for a local Repository",
        )))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(
                    document::array(coco::Branch::document()).description("List of branches"),
                )
                .mime("application/json"),
            )
            .description("List of branches"),
        ))
        .and_then(handler::local_branches)
}

/// `GET /revisions/<project_id>`
fn revisions_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("revisions")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::document(document::description(
            "List both branches and tags",
        )))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(
                    document::array(Revision::document()).description("List of revisions per repo"),
                )
                .mime("application/json"),
            )
            .description("List of branches and tags"),
        ))
        .and_then(handler::revisions)
}

/// `GET /tags/<project_id>`
fn tags_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("tags")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::document(document::description("List Tags")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(document::array(coco::Tag::document()).description("List of tags"))
                    .mime("application/json"),
            )
            .description("List of tags"),
        ))
        .and_then(handler::tags)
}

/// `GET /tree/<project_id>/<revision>/<prefix>`
fn tree_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("tree")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(warp::filters::query::query::<TreeQuery>())
        .and(document::document(
            document::query("revision", document::string()).description("Git revision"),
        ))
        .and(document::document(
            document::query("prefix", document::string())
                .description("Prefix to filter files and folders by"),
        ))
        .and(document::document(document::description("Fetch a Tree")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(coco::Tree::document()).mime("application/json"),
            )
            .description("Tree for path found"),
        ))
        .and_then(handler::tree)
}

/// Source handlers for conversion between core domain and http request fullfilment.
mod handler {
    use librad::paths::Paths;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::path::Tail;
    use warp::{reply, Rejection, Reply};

    use crate::avatar;
    use crate::coco;
    use crate::identity;

    /// Fetch a [`coco::Blob`].
    pub async fn blob(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
        super::BlobQuery { path, revision }: super::BlobQuery,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let blob = coco::blob(&paths, &project_id, revision, path)?;

        Ok(reply::json(&blob))
    }

    /// Fetch the list [`coco::Branch`].
    pub async fn branches(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let branches = coco::branches(&paths, &project_id)?;

        Ok(reply::json(&branches))
    }

    /// Fetch a [`coco::Commit`].
    pub async fn commit(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
        sha1: String,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let commit = coco::commit(&paths, &project_id, &sha1)?;

        Ok(reply::json(&commit))
    }

    /// Fetch the list of [`coco::Commit`] from a branch.
    pub async fn commits(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
        branch: String,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let commits = coco::commits(&paths, &project_id, &branch)?;

        Ok(reply::json(&commits))
    }

    /// Fetch the list [`coco::Branch`] for a local repository.
    pub async fn local_branches(path: Tail) -> Result<impl Reply, Rejection> {
        let branches = coco::local_branches(path.as_str())?;

        Ok(reply::json(&branches))
    }

    /// Fetch the list [`coco::Branch`] and [`coco::Tag`].
    pub async fn revisions(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let branches = coco::branches(&paths, &project_id)?;
        let tags = coco::tags(&paths, &project_id)?;
        let revs = ["cloudhead", "rudolfs", "xla"]
            .iter()
            .map(|handle| super::Revision {
                branches: branches.clone(),
                tags: tags.clone(),
                identity: identity::Identity {
                    id: format!("{}@123abcd.git", handle),
                    metadata: identity::Metadata {
                        handle: (*handle).to_string(),
                        display_name: None,
                        avatar_url: None,
                    },
                    avatar_fallback: avatar::Avatar::from(handle, avatar::Usage::Identity),
                    registered: None,
                    shareable_entity_identifier: format!("{}@123abcd.git", handle),
                },
            })
            .collect::<Vec<super::Revision>>();

        Ok(reply::json(&revs))
    }

    /// Fetch the list [`coco::Tag`].
    pub async fn tags(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let tags = coco::tags(&paths, &project_id)?;

        Ok(reply::json(&tags))
    }

    /// Fetch a [`coco::Tree`].
    pub async fn tree(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
        super::TreeQuery { prefix, revision }: super::TreeQuery,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let tree = coco::tree(&paths, &project_id, revision, prefix)?;

        Ok(reply::json(&tree))
    }
}

/// Bundled query params to pass to the blob handler.
#[derive(Debug, Deserialize)]
pub struct BlobQuery {
    /// Location of the blob in tree.
    path: Option<String>,
    /// Revision to use for the history of the repo.
    revision: Option<String>,
}

/// Bundled query params to pass to the tree handler.
#[derive(Debug, Deserialize)]
pub struct TreeQuery {
    /// Path prefix to query the tree.
    prefix: Option<String>,
    /// Revision to query at.
    revision: Option<String>,
}

/// Bundled response to retrieve both branches and tags for a user repo.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
    /// Owner of the repo.
    identity: identity::Identity,
    /// List of [`coco::Branch`].
    branches: Vec<coco::Branch>,
    /// List of [`coco::Tag`].
    tags: Vec<coco::Tag>,
}

impl ToDocumentedType for Revision {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(2);
        properties.insert("identity".into(), identity::Identity::document());
        properties.insert("branches".into(), document::array(coco::Branch::document()));
        properties.insert("tags".into(), document::array(coco::Tag::document()));

        document::DocumentedType::from(properties).description("Revision")
    }
}

impl Serialize for coco::Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Blob", 3)?;
        state.serialize_field("binary", &self.is_binary())?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("info", &self.info)?;
        state.serialize_field("path", &self.path)?;
        state.end()
    }
}

impl ToDocumentedType for coco::Blob {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "binary".into(),
            document::boolean()
                .description("Flag to indicate if the content of the Blob is binary")
                .example(true),
        );
        properties.insert("content".into(), coco::BlobContent::document());
        properties.insert("info".into(), coco::Info::document());

        document::DocumentedType::from(properties).description("Blob")
    }
}

impl Serialize for coco::BlobContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ascii(content) => serializer.serialize_str(content),
            Self::Binary => serializer.serialize_none(),
        }
    }
}

impl ToDocumentedType for coco::BlobContent {
    fn document() -> document::DocumentedType {
        document::string()
            .description("BlobContent")
            .example("print 'hello world'")
            .nullable(true)
    }
}

impl Serialize for coco::Branch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToDocumentedType for coco::Branch {
    fn document() -> document::DocumentedType {
        document::string().description("Branch").example("master")
    }
}

impl Serialize for coco::Commit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Commit", 6)?;
        state.serialize_field("sha1", &self.sha1.to_string())?;
        state.serialize_field("author", &self.author)?;
        state.serialize_field("summary", &self.summary)?;
        state.serialize_field("description", &self.description())?;
        state.serialize_field("committer", &self.committer)?;
        state.serialize_field("committerTime", &self.committer_time.seconds())?;
        state.end()
    }
}

impl ToDocumentedType for coco::Commit {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(6);
        properties.insert(
            "sha1".into(),
            document::string()
                .description("SHA1 of the Commit")
                .example("1e0206da8571ca71c51c91154e2fee376e09b4e7"),
        );
        properties.insert("author".into(), coco::Person::document());
        properties.insert(
            "summary".into(),
            document::string()
                .description("Commit message summary")
                .example("Add text files"),
        );
        properties.insert(
            "description".into(),
            document::string()
                .description("Commit description text")
                .example("Longer desription of the Commit changes."),
        );
        properties.insert("committer".into(), coco::Person::document());
        properties.insert(
            "committerTime".into(),
            document::string()
                .description("Time of the commit")
                .example("1575283425"),
        );
        document::DocumentedType::from(properties).description("Commit")
    }
}

impl Serialize for coco::Info {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Info", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("objectType", &self.object_type)?;
        state.serialize_field("lastCommit", &self.last_commit)?;
        state.end()
    }
}

impl ToDocumentedType for coco::Info {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Name of the file")
                .example("arrows.txt"),
        );
        properties.insert("objectType".into(), coco::ObjectType::document());
        properties.insert("lastCommit".into(), coco::Commit::document());

        document::DocumentedType::from(properties).description("Info")
    }
}

impl Serialize for coco::ObjectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Blob => serializer.serialize_unit_variant("ObjectType", 0, "BLOB"),
            Self::Tree => serializer.serialize_unit_variant("ObjectType", 1, "TREE"),
        }
    }
}

impl ToDocumentedType for coco::ObjectType {
    fn document() -> document::DocumentedType {
        document::enum_string(vec!["BLOB".to_string(), "TREE".to_string()])
            .description("Object type variants")
            .example(Self::Blob)
    }
}

impl Serialize for coco::Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Person", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("avatar", &self.avatar)?;
        state.end()
    }
}

impl ToDocumentedType for coco::Person {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Name part of the commit signature.")
                .example("Alexis Sellier"),
        );
        properties.insert(
            "email".into(),
            document::string()
                .description("Email part of the commit signature.")
                .example("self@cloudhead.io"),
        );
        properties.insert(
            "avatar".into(),
            document::string()
                .description("Reference (url/uri) to a persons avatar image.")
                .example("https://avatars1.githubusercontent.com/u/40774"),
        );

        document::DocumentedType::from(properties).description("Person")
    }
}

impl Serialize for coco::Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToDocumentedType for coco::Tag {
    fn document() -> document::DocumentedType {
        document::string().description("Tag").example("v0.1.0")
    }
}

impl Serialize for coco::Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Tree", 3)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("entries", &self.entries)?;
        state.serialize_field("info", &self.info)?;
        state.end()
    }
}

impl ToDocumentedType for coco::Tree {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "path".into(),
            document::string()
                .description("Absolute path to the tree object from the repo root.")
                .example("ui/src"),
        );
        properties.insert(
            "entries".into(),
            document::array(coco::TreeEntry::document())
                .description("Entries listed in that tree result."),
        );
        properties.insert("info".into(), coco::Info::document());

        document::DocumentedType::from(properties).description("Tree")
    }
}

impl Serialize for coco::TreeEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Tree", 2)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("info", &self.info)?;
        state.end()
    }
}

impl ToDocumentedType for coco::TreeEntry {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(2);
        properties.insert(
            "path".into(),
            document::string()
                .description("Absolute path to the object from the root of the repo.")
                .example("ui/src/main.ts"),
        );
        properties.insert("info".into(), coco::Info::document());

        document::DocumentedType::from(properties).description("TreeEntry")
    }
}

#[allow(clippy::non_ascii_literal, clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use librad::paths::Paths;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::avatar;
    use crate::coco;
    use crate::identity;

    #[tokio::test]
    async fn blob() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let (platinum_id, _platinum_project) = coco::replicate_platinum(
            &tmp_dir,
            &librad_paths,
            "git-platinum",
            "fixture data",
            "master",
        )
        .unwrap();
        let revision = "master";
        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));

        // Get ASCII blob.
        let path = "text/arrows.txt";
        let res = request()
            .method("GET")
            .path(&format!(
                "/blob/{}?revision={}&path={}",
                platinum_id, revision, path
            ))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::blob(
            &librad_paths,
            &platinum_id.to_string(),
            Some(revision.to_string()),
            Some(path.to_string()),
        )
        .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!({
                "binary": false,
                "content": "  ;;;;;        ;;;;;        ;;;;;
  ;;;;;        ;;;;;        ;;;;;
  ;;;;;        ;;;;;        ;;;;;
  ;;;;;        ;;;;;        ;;;;;
..;;;;;..    ..;;;;;..    ..;;;;;..
 ':::::'      ':::::'      ':::::'
   ':`          ':`          ':`
",
                "info": {
                    "name": "arrows.txt",
                    "objectType": "BLOB",
                    "lastCommit": {
                        "sha1": "1e0206da8571ca71c51c91154e2fee376e09b4e7",
                        "author": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                            "name": "Rūdolfs Ošiņš",
                            "email": "rudolfs@osins.org",
                        },
                        "committer": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                            "name": "Rūdolfs Ošiņš",
                            "email": "rudolfs@osins.org",
                        },
                        "summary": "Add text files",
                        "description": "",
                        "committerTime": 1_575_283_425,
                    },
                },
                "path": "text/arrows.txt",
            })
        );

        // Get binary blob.
        let path = "bin/ls";
        let res = request()
            .method("GET")
            .path(&format!(
                "/blob/{}?revision={}&path={}",
                platinum_id, revision, path
            ))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::blob(
            &librad_paths,
            &platinum_id.to_string(),
            Some(revision.to_string()),
            Some(path.to_string()),
        )
        .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!({
                "binary": true,
                "content": Value::Null,
                "info": {
                    "name": "ls",
                    "objectType": "BLOB",
                    "lastCommit": {
                        "sha1": "19bec071db6474af89c866a1bd0e4b1ff76e2b97",
                        "author": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                            "name": "Rūdolfs Ošiņš",
                            "email": "rudolfs@osins.org",
                        },
                        "committer": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                            "name": "Rūdolfs Ošiņš",
                            "email": "rudolfs@osins.org",
                        },
                        "summary": "Add some binary files",
                        "description": "",
                        "committerTime": 1_575_282_964, },
                },
                "path": "bin/ls",
            })
        );
    }

    #[tokio::test]
    async fn branches() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let (platinum_id, _platinum_project) = coco::replicate_platinum(
            &tmp_dir,
            &librad_paths,
            "git-platinum",
            "fixture data",
            "master",
        )
        .unwrap();
        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));
        let res = request()
            .method("GET")
            .path(&format!("/branches/{}", platinum_id))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::branches(&librad_paths, &platinum_id.to_string()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!(["dev", "master", "rad/contributor", "rad/project"]),
        );
    }

    #[tokio::test]
    async fn commit() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let (platinum_id, _platinum_project) = coco::replicate_platinum(
            &tmp_dir,
            &librad_paths,
            "git-platinum",
            "fixture data",
            "master",
        )
        .unwrap();

        let sha1 = "3873745c8f6ffb45c990eb23b491d4b4b6182f95";

        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));
        let res = request()
            .method("GET")
            .path(&format!("/commit/{}/{}", platinum_id, sha1))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::commit(&librad_paths, &platinum_id.to_string(), sha1).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!({
                "sha1": sha1,
                "author": {
                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/6367167426181048581.svg",
                    "name": "Fintan Halpenny",
                    "email": "fintan.halpenny@gmail.com",
                },
                "committer": {
                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/16701125315436463681.svg",
                    "email": "noreply@github.com",
                    "name": "GitHub",
                },
                "summary": "Extend the docs (#2)",
                "description": "I want to have files under src that have separate commits.\r\nThat way src\'s latest commit isn\'t the same as all its files, instead it\'s the file that was touched last.",
                "committerTime": 1_578_309_972,
            }),
        );
    }

    #[tokio::test]
    async fn commits() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let (platinum_id, _platinum_project) = coco::replicate_platinum(
            &tmp_dir,
            &librad_paths,
            "git-platinum",
            "fixture data",
            "master",
        )
        .unwrap();

        let branch = "master";
        let head = "223aaf87d6ea62eef0014857640fd7c8dd0f80b5";

        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));
        let res = request()
            .method("GET")
            .path(&format!("/commits/{}/{}", platinum_id, branch))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::commits(&librad_paths, &platinum_id.to_string(), branch).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(have.as_array().unwrap().len(), 14);

        let head_commit = coco::commit(&librad_paths, &platinum_id.to_string(), head).unwrap();

        assert_eq!(
            have.as_array().unwrap().first().unwrap(),
            &serde_json::to_value(&head_commit).unwrap(),
            "the first commit is the head of the branch"
        );
    }

    #[tokio::test]
    async fn local_branches() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();

        let path = "../fixtures/git-platinum";
        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));
        let res = request()
            .method("GET")
            .path(&format!("/local-branches/{}", path))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::local_branches(path).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!([
                "dev",
                "master",
                "origin/HEAD",
                "origin/dev",
                "origin/master"
            ]),
        );
    }

    #[tokio::test]
    async fn revisions() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let (platinum_id, _platinum_project) = coco::replicate_platinum(
            &tmp_dir,
            &librad_paths,
            "git-platinum",
            "fixture data",
            "master",
        )
        .unwrap();
        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));
        let res = request()
            .method("GET")
            .path(&format!("/revisions/{}", platinum_id))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = {
            let branches = coco::branches(&librad_paths, &platinum_id.to_string()).unwrap();
            let tags = coco::tags(&librad_paths, &platinum_id.to_string()).unwrap();
            ["cloudhead", "rudolfs", "xla"]
                .iter()
                .map(|handle| super::Revision {
                    branches: branches.clone(),
                    tags: tags.clone(),
                    identity: identity::Identity {
                        id: format!("{}@123abcd.git", handle),
                        metadata: identity::Metadata {
                            handle: (*handle).to_string(),
                            display_name: None,
                            avatar_url: None,
                        },
                        avatar_fallback: avatar::Avatar::from(handle, avatar::Usage::Identity),
                        registered: None,
                        shareable_entity_identifier: format!("{}@123abcd.git", handle),
                    },
                })
                .collect::<Vec<super::Revision>>()
        };

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!([
                {
                    "identity": {
                        "id": "cloudhead@123abcd.git",
                        "metadata": {
                            "handle": "cloudhead",
                            "displayName": Value::Null,
                            "avatarUrl": Value::Null,
                        },
                        "registered": Value::Null,
                        "shareableEntityIdentifier": "cloudhead@123abcd.git",
                        "avatarFallback": {
                            "background": {
                                "r": 24,
                                "g": 105,
                                "b": 216,
                            },
                            "emoji": "🏍",
                        },
                    },
                    "branches": [ "dev", "master", "rad/contributor", "rad/project" ],
                    "tags": [ "v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0" ]
                },
                {
                    "identity": {
                        "id": "rudolfs@123abcd.git",
                        "metadata": {
                            "handle": "rudolfs",
                            "displayName": Value::Null,
                            "avatarUrl": Value::Null,
                        },
                        "registered": Value::Null,
                        "shareableEntityIdentifier": "rudolfs@123abcd.git",
                        "avatarFallback": {
                            "background": {
                                "r": 24,
                                "g": 186,
                                "b": 214,
                            },
                            "emoji": "🛷",
                        },
                    },
                    "branches": [ "dev", "master", "rad/contributor", "rad/project" ],
                    "tags": [ "v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0" ]
                },
                {
                    "identity": {
                        "id": "xla@123abcd.git",
                        "metadata": {
                            "handle": "xla",
                            "displayName": Value::Null,
                            "avatarUrl": Value::Null,
                        },
                        "registered": Value::Null,
                        "shareableEntityIdentifier": "xla@123abcd.git",
                        "avatarFallback": {
                            "background": {
                                "r": 155,
                                "g": 157,
                                "b": 169,
                            },
                            "emoji": "🗻",
                        },
                    },
                    "branches": [ "dev", "master", "rad/contributor", "rad/project" ],
                    "tags": [ "v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0" ]
                },
            ]),
        );
    }

    #[tokio::test]
    async fn tags() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let (platinum_id, _platinum_project) = coco::replicate_platinum(
            &tmp_dir,
            &librad_paths,
            "git-platinum",
            "fixture data",
            "master",
        )
        .unwrap();
        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));
        let res = request()
            .method("GET")
            .path(&format!("/tags/{}", platinum_id))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::tags(&librad_paths, &platinum_id.to_string()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!(["v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0"]),
        );
    }

    #[tokio::test]
    async fn tree() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let (platinum_id, _platinum_project) = coco::replicate_platinum(
            &tmp_dir,
            &librad_paths,
            "git-platinum",
            "fixture data",
            "master",
        )
        .unwrap();

        let revision = "master";
        let prefix = "src";

        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));
        let res = request()
            .method("GET")
            .path(&format!(
                "/tree/{}?revision={}&prefix={}",
                platinum_id, revision, prefix
            ))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::tree(
            &librad_paths,
            &platinum_id.to_string(),
            Some(revision.to_string()),
            Some(prefix.to_string()),
        )
        .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!({
                "path": "src",
                "info": {
                    "name": "src",
                    "objectType": "TREE",
                    "lastCommit": {
                        "sha1": "223aaf87d6ea62eef0014857640fd7c8dd0f80b5",
                        "author": {
                            "avatar":  "https://avatars.dicebear.com/v2/jdenticon/4800695552551917589.svg",
                            "name": "Alexander Simmerl",
                            "email": "a.simmerl@gmail.com",
                        },
                        "committer": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/16701125315436463681.svg",
                            "email": "noreply@github.com",
                            "name": "GitHub",
                        },
                        "summary": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig",
                        "description": "Updated README",
                        "committerTime": 1_584_367_899,
                    },
                },
                "entries": [
                    {
                        "path": "src/Eval.hs",
                        "info": {
                            "name": "Eval.hs",
                            "objectType": "BLOB",
                            "lastCommit": {
                                "sha1": "223aaf87d6ea62eef0014857640fd7c8dd0f80b5",
                                "author": {
                                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/4800695552551917589.svg",
                                    "name": "Alexander Simmerl",
                                    "email": "a.simmerl@gmail.com",
                                },
                        "committer": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/16701125315436463681.svg",
                            "email": "noreply@github.com",
                            "name": "GitHub",
                        },
                                "summary": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig",
                                "description": "Updated README",
                                "committerTime": 1_584_367_899,
                            },
                        },
                    },
                    {
                        "path": "src/memory.rs",
                        "info": {
                            "name": "memory.rs",
                            "objectType": "BLOB",
                            "lastCommit": {
                                "sha1": "e24124b7538658220b5aaf3b6ef53758f0a106dc",
                                "author": {
                                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                                    "name": "Rūdolfs Ošiņš",
                                    "email": "rudolfs@osins.org",
                                },
                                "committer": {
                                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                                    "name": "Rūdolfs Ošiņš",
                                    "email": "rudolfs@osins.org",
                                },
                                "summary": "Move examples to \"src\"",
                                "description": "",
                                "committerTime": 1_575_283_266,
                            },
                        },
                    },
                ],
            }),
        );
    }
}