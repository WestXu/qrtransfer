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

async function beepN(n) {
    function sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
    for (let i = 0; i < n; i++) {
        beep();
        await sleep(50);
    }
}

function add_download(base64_data, file_name) {
    let a = document.createElement("a");
    a.href = "data:;base64," + base64_data;
    a.download = file_name;
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
            var decoder = window.qrtransfer.new_decoder();
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
                        beepN(counter);
                        if (decoder.is_finished()) {
                            stop_receiving();
                            var finished = window.decoder.get_finished();
                            add_download(finished.to_base64(), finished.get_name());
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

