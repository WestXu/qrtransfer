import base64
import io
import tempfile
from pathlib import Path
from typing import Union

import qrcode
from tqdm.auto import tqdm


class QrSender:
    def __init__(self, file: Union[str, Path]):
        self.file = Path(file)
        self.data = self.file.read_bytes()

        size = 100
        self.chunks = [self.data[i : i + size] for i in range(0, len(self.data), size)]

    @staticmethod
    def make_qr(data: bytes) -> str:
        img = qrcode.make(data)
        buffer = io.BytesIO()
        img.save(buffer)
        return base64.b64encode(buffer.getvalue()).decode()

    @staticmethod
    def mk_html_img(data: bytes, i: int) -> str:
        return (
            f'<table border="1" style="float:left;font-size:30">'
            f'<tr><td><img src="data:image/png;base64,{QrSender.make_qr(data)}"></td></tr>'
            f'<tr><td align="center">{i}</td></tr></table>'
        )

    def save_html(self, p: Path) -> int:
        data_length = p.write_text(
            "".join(
                [
                    self.mk_html_img(chunk, i)
                    for i, chunk in enumerate(tqdm(self.chunks))
                ]
            )
        )
        return data_length

    def show(self):
        with tempfile.NamedTemporaryFile(mode='r', suffix='.html') as f:
            self.save_html(Path(f.name))
            print(f.name)
            input('Press any key to delete it...')


if __name__ == "__main__":
    QrSender('tmp/example.png').show()
