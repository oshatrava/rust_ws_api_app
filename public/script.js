const ws = new WebSocket("ws://127.0.0.1:8000/ws");

const sendBtn = document.querySelector('#btn');
const messages = document.querySelector('#messages');
const statusBadge = document.querySelector("#status");

ws.onopen = function(evt) {
    console.log("[open] Connection established!");
    updateStatusBadge(evt.type);
};

ws.onclose = function(evt) {
    if (evt.wasClean) {
        console.log(`[close] Connection closed cleanly! Code: ${evt.code} / ${evt.reason}`);
    } else {
        console.log("[close] Connection died!");
    }
    updateStatusBadge(evt.type);
};

ws.onerror = function(error) {
    console.log(`[error]`, error);
};

ws.onmessage = function(evt) {
    const msgEl = document.createElement("li");
    msgEl.classList = 'list-group-item';
    msgEl.textContent = evt.data;
    messages.append(msgEl);
};

function updateStatusBadge(status) {
    const statusBadge = document.querySelector("#status");
    if (status === 'open') {
        statusBadge.innerHTML = "Online";
        statusBadge.classList = "badge text-bg-success";
    } else {
        statusBadge.innerHTML = "Offline";
        statusBadge.classList = "badge text-bg-secondary";
    }
};

sendBtn.addEventListener('click', () => {
    const input = document.querySelector('#input');

    const message = input.value.trim();
    if (!message) {
        return;
    }
    ws.send(JSON.stringify({
        username: message,
        channel: "root",
    }));
    input.value = "";
})