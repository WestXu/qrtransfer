import base64
import hashlib
from pathlib import Path
from typing import Dict, List, Set


class cached_property:
    def __init__(self, func):
        self.__doc__ = getattr(func, "__doc__")
        self.func = func

    def __get__(self, obj, cls):
        if obj is None:
            return self

        value = obj.__dict__[self.func.__name__] = self.func(obj)
        return value


class Encoder:
    def __init__(self, file: Path) -> None:
        self.file = Path(file)

        self.file_name = self.file.name
        self.data = self.file.read_bytes()

    @cached_property
    def chunks(self) -> List[bytes]:
        size = 100
        return [self.data[i : i + size] for i in range(0, len(self.data), size)]

    @cached_property
    def headers(self) -> Dict[str, bytes]:
        final_hash = hashlib.sha1(self.data).hexdigest()
        return {
            'NAME': f'NAME:{self.file_name}'.encode(),
            'LEN': f'LEN:{len(self.chunks)}'.encode(),
            'HASH': f'HASH:{final_hash}'.encode(),
        }

    @cached_property
    def payloads(self) -> Dict[str, bytes]:
        data_payloads = {
            str(
                counter + 1
            ): f'{counter + 1}:{base64.b64encode(data).decode("utf-8")}'.encode()
            for counter, data in enumerate(self.chunks)
        }
        payloads = {**self.headers, **data_payloads}
        return payloads


class Decoder:
    def __init__(self):
        self.expected_iterations = None
        self.received_iterations: Dict[bytes, bytes] = {}
        self.file_name = None
        self.hash = None
        self.lengh = None

    def set_name(self, name: str):
        print(f'[*] File name: {name}')
        self.file_name = name

    def set_length(self, length: int):
        print(f'[*] The message will come in {length} parts')
        self.lengh = length
        self.expected_iterations: Set[bytes] = {b'NAME', b'LEN', b'HASH'} | {
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

    @cached_property
    def data(self):
        assert self.is_finished
        data = b''.join(
            [
                base64.b64decode(data)
                for i, data in sorted(
                    self.received_iterations.items(),
                    key=lambda _: int(_[0])
                    if _[0] not in {b'NAME', b'LEN', b'HASH'}
                    else 0,
                )
                if i not in {b'NAME', b'LEN', b'HASH'}
            ]
        )
        return data

    def check_integrity(self):
        assert self.is_finished

        final_hash = hashlib.sha1(self.data).hexdigest()

        if final_hash != self.hash:
            raise ValueError(f'[*] Expected: {self.hash}, got: {final_hash}')

    def save_file(self, folder: Path = Path('.')):
        file = folder / self.file_name
        file.write_bytes(self.data)
        print(f'File saved to {file}')

    def process_chunk(self, chunk: bytes):
        i, data = chunk.split(b':', maxsplit=1)
        if i in self.received_iterations:
            return False

        self.received_iterations[i] = data

        if i == b'NAME':
            self.set_name(data.decode())

        if i == b'LEN':
            self.set_length(int(data))

        if i == b'HASH':
            self.set_hash(data.decode())

        return True


if __name__ == '__main__':
    import random

    encoder = Encoder(Path('data.py'))
    payloads = list(encoder.payloads.values())
    random.shuffle(payloads)

    decoder = Decoder()
    for data in payloads:
        assert not decoder.is_finished
        decoder.process_chunk(data)

    assert decoder.is_finished
    decoder.check_integrity()
