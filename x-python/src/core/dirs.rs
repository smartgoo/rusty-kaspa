use std::path::PathBuf;
use kaspad_lib::daemon::{get_app_dir, CONSENSUS_DB, DEFAULT_DATA_DIR, META_DB, UTXOINDEX_DB};
 
pub struct Dirs {
    pub app_dir: PathBuf,
    pub network_dir: PathBuf,
    pub db_dir: PathBuf,
    pub utxo_index_db_dir: Option<PathBuf>,
    pub meta_db_dir: PathBuf,
    pub consensus_db_dir: PathBuf,
}

impl Dirs {
    pub fn new(app_dir: Option<PathBuf>, network: Option<String>) -> Self {
        // Set app_dir. Use default if nothing passed
        let app_dir = match app_dir {
            Some(dir) => dir,
            None => get_app_dir(),
        };

        let network_dir = match network.as_deref() {
            Some("mainnet") => app_dir.join("kaspa-mainnet"),
            Some("testnet") => app_dir.join("kaspa-testnet"),
            Some("devnet") => app_dir.join("kaspa-devnet"),
            Some("simnet") => app_dir.join("kaspa-simnet"),
            _ => app_dir.join("kaspa-mainnet"),
        };

        let db_dir = network_dir.join(DEFAULT_DATA_DIR);
        let utxo_index_db_dir = if db_dir.join(UTXOINDEX_DB).exists() { Some(db_dir.join(UTXOINDEX_DB)) } else { None };
        let meta_db_dir = db_dir.join(META_DB);
        let consensus_db_dir = db_dir.join(CONSENSUS_DB);

        Dirs {
            app_dir,
            network_dir,
            db_dir,
            utxo_index_db_dir,
            meta_db_dir,
            consensus_db_dir
        }
    }

    pub fn validate_existence(&self) -> bool {
        self.app_dir.exists() &&
        self.network_dir.exists() &&
        self.db_dir.exists() &&
        self.utxo_index_db_dir.as_ref().map_or(true, |dir| dir.exists()) &&
        self.meta_db_dir.exists() &&
        self.consensus_db_dir.exists()
    }
}
