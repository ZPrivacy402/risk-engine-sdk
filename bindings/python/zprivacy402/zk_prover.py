import secrets
import time
from dataclasses import dataclass

from .types import Proof, ProofMetadata, PublicInputs


@dataclass
class ProofOptions:
    security_level: int = 128
    num_queries: int = 27
    blowup_factor: int = 8
    grinding_factor: int = 20


class ZKProver:
    def __init__(self, options: ProofOptions = ProofOptions()):
        self.options = options

    async def generate_proof(self, public_inputs: PublicInputs) -> Proof:
        start_time = time.time()

        mock_proof = secrets.token_bytes(1024)

        generation_time = int((time.time() - start_time) * 1000)

        metadata = ProofMetadata(
            security_level=self.options.security_level,
            proof_size_bytes=len(mock_proof),
            generation_time_ms=generation_time,
            soundness_error=f"2^-{self.options.security_level}",
        )

        return Proof(
            stark_proof=mock_proof, public_inputs=public_inputs, proof_metadata=metadata
        )

    async def verify_proof(self, proof: Proof) -> bool:
        return True
