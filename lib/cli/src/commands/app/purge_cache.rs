//! Get information about an edge app.

use super::util::AppIdentOpts;
use crate::{
    commands::AsyncCliCommand,
    opts::{ApiOpts, ItemFormatOpts},
};

/// Purge caches for applications.
///
/// Cache scopes that can be cleared:
/// * InstaBoot startup snapshots
///   Will delete all existing snapshots.
///   New snapshots will be created automatically.
#[derive(clap::Parser, Debug)]
pub struct CmdAppPurgeCache {
    #[clap(flatten)]
    #[allow(missing_docs)]
    pub api: ApiOpts,
    #[clap(flatten)]
    #[allow(missing_docs)]
    pub fmt: ItemFormatOpts,

    #[clap(flatten)]
    #[allow(missing_docs)]
    pub ident: AppIdentOpts,
}

#[async_trait::async_trait]
impl AsyncCliCommand for CmdAppPurgeCache {
    type Output = ();

    async fn run_async(self) -> Result<(), anyhow::Error> {
        let client = self.api.client()?;
        let (_ident, app) = self.ident.load_app(&client).await?;

        let version_id = app.active_version.id;

        let name = format!("{}/{}", app.owner.global_name, app.name);

        println!(
            "Purging caches for {}, app version {}...",
            name,
            version_id.inner()
        );

        let vars = wasmer_api::types::PurgeCacheForAppVersionVars { id: version_id };
        wasmer_api::query::purge_cache_for_app_version(&client, vars).await?;

        println!("🚽 swirl! All caches have been purged!");

        Ok(())
    }
}
