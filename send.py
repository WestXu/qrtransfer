import base64
import hashlib
from functools import cached_property
from typing import Dict, List

from browser import bind, document, window


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


def draw_qr(dom_id, payload):
    print(dom_id, payload)
    dom = document[dom_id]
    dom.innerHTML = ""
    window.QRCode.new(dom, payload)


def encode(file_name, int_array) -> Dict[str, str]:
    return Encoder(file_name, int_array).payloads


def mk_html_img(payload: bytes, name: str) -> str:
    return (
        f'<table style="float:left;font-size:30">'
        f'<tr><td class="qrtd" id="qr-{name}"></td></tr>'
        f'<tr><td align="center">{name}</td></tr></table>'
    )


@bind(document["file-selector"], "change")
def read_file_content(ev):
    def onload(event):
        buffer = event.target.result
        int_array = window.array_from(window.Uint8Array.new(buffer))
        encoded = encode(document["file-selector"].files[0].name, int_array)

        qr_html = "".join(
            [mk_html_img(payload, name) for name, payload in encoded.items()]
        )
        window.qr_html = qr_html
        document["qrcode"].innerHTML = qr_html

        for name, payload in encoded.items():
            draw_qr(f"qr-{name}", payload)

    reader = window.FileReader.new()
    reader.bind("load", onload)
    reader.readAsArrayBuffer(document["file-selector"].files[0])
