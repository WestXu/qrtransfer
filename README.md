# qrtransfer

This is a proof-of-concept project, implemented in Rust WebAssembly.

## How to Use

![Demo](demo.gif)

1. Open [qrtransfer.westxu.com](https://qrtransfer.westxu.com) on one of your device, select file, wait for it to be processed.
2. Open [qrtransfer.westxu.com](https://qrtransfer.westxu.com) on your another device, scan using the camera until finishing scanning all the qrcodes, and then save the reassembled file.

## FAQ

#### How does it work?

Sender:

* Read the file into binary data
* Compress it
* Hash it
* Split it into a lot of chunks
* Encode each chunk into a small qrcode
* Play them one by one

Receiver:

* Scan all the qrcodes
* Reassemble the chunks into one
* Validate the hash
* Decompress
* Save the file

#### Privacy?

The website is static, hosted on github pages, and auto-deployed by github actions. The file will be processed natively in your browser and won't be uploaded to any server. Try turn off your wifi before selecting the file.

#### Offline version?

Check [release](https://github.com/WestXu/qrtransfer/releases).

#### Transfer speed?

~1KB/s. I know, but it works.
