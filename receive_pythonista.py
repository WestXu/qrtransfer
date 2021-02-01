from ctypes import c_void_p

import sound
import ui
from objc_util import *

from data import Decoder

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
            if decoder.lengh is not None
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
