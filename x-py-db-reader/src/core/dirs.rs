use std::path::PathBuf;

pub struct Dirs {
    pub home_dir: PathBuf,
    pub app_dir: PathBuf,
    pub network_dir: PathBuf,
    pub db_dir: PathBuf,
    pub utxo_index_db_dir: Option<PathBuf>,
    pub meta_db_dir: PathBuf,
    pub consensus_db_dir: PathBuf,
}

impl Dirs {
    pub fn new(app_dir: Option<PathBuf>, network: Option<String>) -> Self {
        // TODO check if app_dir exists on system and throw error if not

        // Set home dir based on OS
        let home_dir = Self::get_home_dir();

        // Set app_dir. Use passed param, if one exists, or default app dir
        let app_dir = match app_dir {
            Some(dir) => dir,
            None => Self::get_app_dir(),
        };

        // Set network_dir based on passed network
        let network_dir = match network.as_deref() {
            Some("mainnet") => app_dir.join("kaspa-mainnet"),
            Some("testnet") => app_dir.join("kaspa-testnet"),
            Some("devnet") => app_dir.join("kaspa-devnet"),
            Some("simnet") => app_dir.join("kaspa-simnet"),
            _ => app_dir.join("kaspa-mainnet"),
        };

        // Set db_dir
        let db_dir = network_dir.join("datadir");

        // Set utxo_index_db_dir if utxoindex dir exists inside of db_dir
        let utxo_index_db_dir = if db_dir.join("utxoindex").exists() { Some(db_dir.join("utxoindex")) } else { None };

        // Set meta_db_dir
        let meta_db_dir = db_dir.join("meta");

        // Set consensus_db_dir
        let consensus_db_dir = db_dir.join("consensus");

        Dirs {
            home_dir,
            app_dir,
            network_dir,
            db_dir,
            utxo_index_db_dir,
            meta_db_dir,
            consensus_db_dir
        }
    }

    fn get_home_dir() -> PathBuf {
        #[cfg(target_os = "windows")]
        return dirs::data_local_dir().unwrap();
        #[cfg(not(target_os = "windows"))]
        return dirs::home_dir().unwrap();
    }

    fn get_app_dir() -> PathBuf {
        #[cfg(target_os = "windows")]
        return Self::get_home_dir().join("rusty-kaspa");
        #[cfg(not(target_os = "windows"))]
        return Self::get_home_dir().join(".rusty-kaspa");
    }

    // fn get_out_dir() -> PathBuf {
    //     let outdir = Self::get_home_dir().join("rusty-kaspa-out");
    //     if !outdir.exists() {
    //         let _ = fs::create_dir_all(&outdir);
    //     }
    //     return outdir;
    // }
}
