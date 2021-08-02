import base64
import hashlib
from functools import cached_property
from typing import Dict, List


class Encoder:
    def __init__(self, file_name: str, data: List[int]) -> None:
        self.file_name = file_name
        self.data: bytes = bytes(data)

    @cached_property
    def chunks(self) -> List[bytes]:
        size = 100
        return [self.data[i : i + size] for i in range(0, len(self.data), size)]

    @cached_property
    def headers(self) -> Dict[str, str]:
        final_hash = hashlib.sha1(self.data).hexdigest()
        return {
            'NAME': f'NAME:{self.file_name}',
            'LEN': f'LEN:{len(self.chunks)}',
            'HASH': f'HASH:{final_hash}',
        }

    @cached_property
    def payloads(self) -> Dict[str, str]:
        data_payloads = {
            str(counter + 1): f'{counter + 1}:{base64.b64encode(data).decode("utf-8")}'
            for counter, data in enumerate(self.chunks)
        }
        payloads = {**self.headers, **data_payloads}
        return payloads
