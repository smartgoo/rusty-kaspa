##########################################
#
#  WARNING - this is out of date.
#
##########################################
from typing import Optional


class DBReader:
    """
    A class for reading rusty-kaspa's various RocksDB instances. Primary interface of this package.
    """
    home_dir: str
    app_dir: str
    network_dir: str 
    db_dir: str
    utxo_index_db_dir: str
    meta_db_dir: str
    consensus_db_dir: str

    stores: Stores

    def __init__(app_dir: Optional[str], network: Optional[str]) -> None:
        """
        :param app_dir: Filepath of your rusty-kaspa appdir. Optional, default is based on OS. For Windows, <your homedir>/rusty-kaspa. For Linux, <your homedir>/.rusty-kaspa
        :param network: Kaspa network. Optional, default is mainnet. Options are mainnet, testnet, devnet, simnet.
        """

    def get_current_consensus_entry() -> int:
        """
        Reads the meta store and returns the current consensus key.
        """

class Stores:
    """
    A collection of Python-wrapped rusty-kaspa stores.
    """
    # metadata: PyMetadataStore TODO
    headers: HeadersStore

    # Optional UTXO index stores
    circulating_supply: CirculatingSupplyStore
    utxo_index: UtxoIndexStore
    utxo_index_tips: UtxoIndexTipsStore


class HeadersStore:
    """
    A class for reading rusty-kaspa's UtxoIndexTipsStore. Node must be ran with --utxoindex.
    """

    def get(block_hash: str) -> Optional[dict]:
        """
        Reads the block header store for a given block.

        Returns None if block not found.
        If block is found, returns a dict with the following keys and value types:
            - 'hash': str
            - 'version': int
            - 'parents_by_level': list[list[str]]
            - 'hash_merkle_root': str
            - 'accepted_id_merkle_root': str
            - 'utxo_commitment': str
            - 'timestamp': int
            - 'bits': int
            - 'nonce': int
            - 'daa_score': int
            - 'blue_score':
            - 'pruning_point': str
        """


class CirculatingSupplyStore:
    """
    A class for reading rusty-kaspa's CirculatingSupplyStore. Node must be ran with --utxoindex.
    """

    def get() -> int:
        """
        Returns the circulating supply in sompi.
        """


class UtxoIndexStore:
    """
    A class for reading rusty-kaspa's UtxoSetByScriptPublicKeyStore. Node must be ran with --utxoindex.
    """
 
    def export(
        filepath: str,
        address: bool = True,
        daa_score: bool = True,
        amount: bool = True,
        is_coinbase: bool = True,
        outpoint: bool = False,
        chunk_size: int = 100_000,
        verbose: bool = False,
    ) -> int:
        """
        Exports the entire utxo set to a local CSV file. THIS FILE WILL BE LARGE! Node must be ran with --utxoindex.

        :param filepath: The file path to export to.
        :param address: Whether to include address in the file. Defaults to True.
        :param daa_score: Whether to include daa score in the file. Defaults to True.
        :param amount: Whether to include amount in the file. Defaults to True.
        :param is_coinbase:  Whether to include is_coinbase in the file. Defaults to True.
        :param outpoint: Whether to include outpoint (transaction_id and transaction_index) in the file. Defaults to False.
        :param chunk_size: Chunk size used for iterative over utxo set. Default is 100,000.
        :param verbose: Print progress while writing the file.
        """


class UtxoIndexTipsStore:
    """
    A class for reading rusty-kaspa's UtxoIndexTipsStore. Node must be ran with --utxoindex.
    """

    def get() -> list[str]:
        """
        Returns utxo index tips (block hashes).
        """