function read_file_content() {
    const fileSelector = document.getElementById("file-selector")
    progress_div = document.getElementById("progress")
    progress_div.innerText = "Processing..."


    const reader = new FileReader();
    reader.addEventListener('load', (ev) => {
        buffer = ev.target.result
        int_array = Array.from(new Uint8Array(buffer))
        file_name = fileSelector.files[0].name
        window.send(file_name, int_array)
        progress_div.innerText = "Finished."
    });
    reader.readAsArrayBuffer(fileSelector.files[0])
}
