function read_file_content() {
    const fileSelector = document.getElementById("file-selector")
    progress_div = document.getElementById("progress")
    progress_div.innerText = "Processing..."


    const reader = new FileReader();
    reader.addEventListener('load', (ev) => {
        buffer = ev.target.result
        int_array = Array.from(new Uint8Array(buffer))
        file_name = fileSelector.files[0].name
        window.qrtransfer.send(file_name, int_array)
        document.getElementById("scroll-check-div").style.display = "block";
    });
    reader.readAsArrayBuffer(fileSelector.files[0])
}

function scroll() {
    window.scrollBy({
        top: 200,
        behavior: 'instant'
    });
}

function start_scroll() {
    window.scrollIntervalId = setInterval(
        scroll,
        1000
    );
    console.log("start_scroll");
}

function stop_scroll() {
    clearInterval(window.scrollIntervalId);
    console.log("stop_scroll");
}

function toggle_scroll() {
    if (document.getElementById("scroll-check").checked) {
        start_scroll();
    } else {
        stop_scroll();
    }
}
