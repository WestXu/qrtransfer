import base64
import hashlib
from ctypes import c_void_p
from functools import cached_property
from pathlib import Path
from typing import Dict, Optional, Set

import sound
import ui
from objc_util import *


class Decoder:
    def __init__(self):
        self.expected_iterations: Optional[Set[bytes]] = None
        self.received_iterations: Dict[bytes, bytes] = {}
        self.file_name = None
        self.hash = None
        self.length = None

    def set_name(self, name: str):
        print(f'[*] File name: {name}')
        self.file_name = name

    def set_length(self, length: int):
        print(f'[*] The message will come in {length} parts')
        self.length = length
        self.expected_iterations = {b'NAME', b'LEN', b'HASH'} | {
            str(i + 1).encode() for i in range(length)
        }

    def set_hash(self, hash: str):
        print(f'[*] Hash {hash}')
        self.hash = hash

    @property
    def expecting(self) -> Set[bytes]:
        assert self.expected_iterations is not None
        return self.expected_iterations - set(self.received_iterations.keys())

    @property
    def is_finished(self) -> bool:
        if self.length is None:
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


decoder = Decoder()
main_view = None

AVCaptureSession = ObjCClass('AVCaptureSession')
AVCaptureDevice = ObjCClass('AVCaptureDevice')
AVCaptureDeviceInput = ObjCClass('AVCaptureDeviceInput')
AVCaptureMetadataOutput = ObjCClass('AVCaptureMetadataOutput')
AVCaptureVideoPreviewLayer = ObjCClass('AVCaptureVideoPreviewLayer')
dispatch_get_current_queue = c.dispatch_get_current_queue
dispatch_get_current_queue.restype = c_void_p


def captureOutput_didOutputMetadataObjects_fromConnection_(
    _self, _cmd, _output, _metadata_objects, _conn
):
    objects = ObjCInstance(_metadata_objects)
    for obj in objects:
        try:
            chunk = str(obj.stringValue()).encode()
            if decoder.process_chunk(chunk):
                sound.play_effect('digital:PowerUp7')
        except Exception as e:
            print(e)
        main_view['label'].text = (
            str(
                sorted(
                    [
                        int(_)
                        for _ in decoder.expecting
                        if _ not in {b'NAME', b'LEN', b'HASH'}
                    ]
                )
            )
            if decoder.length is not None
            else 'No length got.'
        )


MetadataDelegate = create_objc_class(
    'MetadataDelegate',
    methods=[captureOutput_didOutputMetadataObjects_fromConnection_],
    protocols=['AVCaptureMetadataOutputObjectsDelegate'],
)


@on_main_thread
def main():
    global main_view
    delegate = MetadataDelegate.new()
    main_view = ui.View(frame=(0, 0, 800, 800))
    main_view.name = 'Barcode Scanner'
    session = AVCaptureSession.alloc().init()
    device = AVCaptureDevice.defaultDeviceWithMediaType_('vide')
    _input = AVCaptureDeviceInput.deviceInputWithDevice_error_(device, None)
    if _input:
        session.addInput_(_input)
    else:
        print('Failed to create input')
        return
    output = AVCaptureMetadataOutput.alloc().init()
    queue = ObjCInstance(dispatch_get_current_queue())
    output.setMetadataObjectsDelegate_queue_(delegate, queue)
    session.addOutput_(output)
    output.setMetadataObjectTypes_(output.availableMetadataObjectTypes())
    prev_layer = AVCaptureVideoPreviewLayer.layerWithSession_(session)
    prev_layer.frame = ObjCInstance(main_view).bounds()
    prev_layer.setVideoGravity_('AVLayerVideoGravityResizeAspectFill')
    ObjCInstance(main_view).layer().addSublayer_(prev_layer)
    label = ui.Label(frame=(0, 0, 800, 30), flex='W', name='label')
    label.background_color = (0, 0, 0, 0.5)
    label.text_color = 'white'
    label.text = 'Nothing scanned yet'
    label.alignment = ui.ALIGN_CENTER
    main_view.add_subview(label)
    session.startRunning()
    main_view.present('sheet')
    main_view.wait_modal()
    session.stopRunning()
    delegate.release()
    session.release()
    output.release()

    decoder.check_integrity()
    decoder.save_file()


if __name__ == '__main__':
    main()
