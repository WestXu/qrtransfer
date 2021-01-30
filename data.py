import hashlib
from functools import cached_property
from pathlib import Path
from typing import Dict, List, Set


class Encoder:
    def __init__(self, data: bytes) -> None:
        self.data = data

    @cached_property
    def chunks(self) -> List[bytes]:
        size = 100
        return [self.data[i : i + size] for i in range(0, len(self.data), size)]

    @cached_property
    def headers(self) -> Dict[str, bytes]:
        final_hash = hashlib.sha1(self.data).hexdigest()
        return {
            'LEN': f'LEN:{len(self.chunks)}'.encode(),
            'HASH': f'HASH:{final_hash}'.encode(),
        }

    @cached_property
    def payloads(self) -> Dict[str, bytes]:
        data_payloads = {
            str(counter + 1): f"{counter + 1}:".encode() + data
            for counter, data in enumerate(self.chunks)
        }
        payloads = {**self.headers, **data_payloads}
        return payloads


class Decoder:
    def __init__(self):
        self.expected_iterations = None
        self.received_iterations: Dict[bytes, bytes] = {}
        self.hash = None
        self.lengh = None

    def set_length(self, length: int):
        print(f'[*] The message will come in {length} parts')
        self.lengh = length
        self.expected_iterations: Set[bytes] = {b'LEN', b'HASH'} | {
            str(i + 1).encode() for i in range(length)
        }

    def set_hash(self, hash: str):
        print(f'[*] Hash {hash}')
        self.hash = hash

    @property
    def expecting(self) -> Set[bytes]:
        assert self.lengh is not None
        return self.expected_iterations - set(self.received_iterations.keys())

    @property
    def is_finished(self) -> bool:
        if self.lengh is None:
            return False
        return len(self.expecting) == 0

    def check_integrity(self):
        assert self.is_finished

        data = b''.join(
            [
                data
                for i, data in sorted(
                    self.received_iterations.items(),
                    key=lambda _: int(_[0]) if _ in {b'LEN', b'HASH'} else 0,
                )
                if i not in {b'LEN', b'HASH'}
            ]
        )
        final_hash = hashlib.sha1(data).hexdigest()

        if final_hash != self.hash:
            raise ValueError(f'[*] Expected: {self.hash}, got: {final_hash}')

    def process_chunk(self, chunk: bytes):
        i, data = chunk.split(b':', maxsplit=1)
        if i in self.received_iterations:
            return False

        self.received_iterations[i] = data

        if i == b'LEN':
            self.set_length(int(data))

        if i == b'HASH':
            self.set_hash(data.decode())

        return True


if __name__ == '__main__':
    encoder = Encoder(Path('tmp/example.jpg').read_bytes())
    decoder = Decoder()
    for _, data in encoder.payloads.items():
        assert not decoder.is_finished
        decoder.process_chunk(data)

    assert decoder.is_finished
    decoder.check_integrity()
