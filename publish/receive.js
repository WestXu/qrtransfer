let context = new AudioContext();
const beep = (freq = 1500, duration = 30, vol = 100) => {
    const oscillator = context.createOscillator();
    const gain = context.createGain();
    oscillator.connect(gain);
    oscillator.frequency.value = freq;
    oscillator.type = "square";
    gain.connect(context.destination);
    gain.gain.value = vol * 0.01;
    oscillator.start(context.currentTime);
    oscillator.stop(context.currentTime + duration * 0.001);
}

function add_download(base64_data) {
    let a = document.createElement("a");
    a.href = "data:;base64," + base64_data;
    a.download = window.decoder.get_name();
    a.innerText = "Download";
    document.getElementById("receive").appendChild(a);
    a.click();
}

function start_receiving() {
    navigator.mediaDevices.getUserMedia({ video: { facingMode: "environment" } }).then(
        function (stream) {
            var video = document.getElementById('scan-video');
            var canvas = document.getElementById('canvas');
            var camQrResult = document.getElementById('cam-qr-result');
            var ctx = canvas.getContext('2d');
            var decoder = window.Decoder.new();
            window.decoder = decoder;

            video.srcObject = window.stream = stream;
            video.onloadedmetadata = () => {
                window.intervalId = setInterval(() => {
                    canvas.width = video.videoWidth;
                    canvas.height = video.videoHeight;
                    ctx.drawImage(video, 0, 0);
                    var myImageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
                    let counter = decoder.scan(
                        canvas.width, canvas.height, Array.from(myImageData.data)
                    );
                    if (counter > 0) {
                        camQrResult.textContent = window.decoder.get_progress();
                        for (let i = 0; i < counter; i++) {
                            beep();
                        }
                        if (decoder.is_finished()) {
                            stop_receiving();
                            add_download(window.decoder.to_base64());
                        }
                    }
                }, 40);
            };
        }
    )
}

function stop_receiving() {
    window.stream.getTracks().forEach(function (track) {
        track.stop();
    });
    clearInterval(window.intervalId);
    var canvas = document.getElementById('canvas');
    var ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
}

