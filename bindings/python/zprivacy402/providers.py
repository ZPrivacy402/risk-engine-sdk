from dataclasses import dataclass
from typing import Optional

from .types import Proof


@dataclass
class SolanaProviderConfig:
    rpc_url: str
    program_id: Optional[str] = None


class SolanaProvider:
    def __init__(self, config: SolanaProviderConfig):
        self.config = config

    async def submit_transaction(self, proof: Proof, payer_keypair: bytes) -> str:
        signature = f"solana_tx_{int(time.time() * 1000)}"
        return signature

    async def get_balance(self, address: str) -> float:
        return 1000.0

    async def verify_on_chain(self, proof: Proof) -> bool:
        return True
