from browser import bind, document, window

from encoder import Dict, Encoder


def draw_qr(dom_id, payload):
    print(dom_id, payload)
    dom = document[dom_id]
    dom.innerHTML = ""
    window.QRCode.new(dom, payload)


def encode(file_name, int_array) -> Dict[str, str]:
    return Encoder(file_name, int_array).payloads


def mk_html_img(payload: bytes, name: str) -> str:
    return (
        f'<table border="1" style="float:left;font-size:30">'
        f'<tr><td id="qr-{name}"></td></tr>'
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
