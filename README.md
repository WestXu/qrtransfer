# qrtransfer

[![pypi](https://flat.badgen.net/pypi/v/qrtransfer)](https://pypi.org/project/qrtransfer/)

## Installation

You can install from pypi using `pip install qrtransfer`, or install from source using `python3 setup.py install`

## How to Use

![Demo](demo.gif)

1. Copy `receive_pythonista.py` & `qrtransfer/data.py` into your iPad/iPhone's pythonista app.
2. In any folder, run `qrtransfer [file]` in terminal, an `[file].html` which contains a matrix of qrcodes encoded from the file's binary will be generated into the same folder.
3. Open the html with any browser you like, zoom/adjust the window as you wish, run the `receive_pythonista.py` script in pythonista, scan using your camera while scrolling/auto-scrolling your browser until finish scanning all qrcodes, view/share the received file in pythonista.

## Note

The transfer speed is ~0.1KB/s. I know, but it works.
