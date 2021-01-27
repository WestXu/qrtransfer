import base64
import hashlib
from functools import cached_property
from typing import Dict, List

MESSAGE_BEGIN = b'-----BEGIN QR MESSAGE-----'
MESSAGE_END = b'-----END QR MESSAGE-----'
HEADER_BEGIN = b'-----BEGIN QR HEADER-----'
HEADER_END = b'-----END QR HEADER-----'


class Encoder:
    def __init__(self, data: bytes) -> None:
        self.data = data

    @cached_property
    def chunks(self) -> List[bytes]:
        size = 100
        return [self.data[i : i + size] for i in range(0, len(self.data), size)]

    @cached_property
    def headers(self) -> Dict[str, bytes]:
        return {
            'MESSAGE_BEGIN': MESSAGE_BEGIN,
            'HEADER_BEGIN': HEADER_BEGIN,
            'LEN': f'LEN:{len(self.chunks)}'.encode(),
            'HASH': f'HASH:{hashlib.sha1(self.data).hexdigest()}'.encode(),
            'HEADER_END': HEADER_END,
        }

    @cached_property
    def payloads(self) -> Dict[str, bytes]:
        data_payloads = {
            str(
                counter + 1
            ): f'{counter + 1:010d}:{base64.b64encode(data).decode("utf-8")}'.encode()
            for counter, data in enumerate(self.chunks)
        }
        payloads = {**self.headers, **data_payloads, **{'MESSAGE_END': MESSAGE_END}}
        return payloads
