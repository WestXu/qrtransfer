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
