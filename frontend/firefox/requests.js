const SERVER_IP = "https://dev01.playground.extension.42heilbronn.de/api";

//listens for a message from the content script and sends a request to the server depending on the message
browser.runtime.onMessage.addListener((request, sender) => {
    if (request == "auth")
        return auth();
    else if (request == "miss")
        return get_missing();
    else if (request.uri.endsWith("info"))
        return get_details(request.uri);
    else if (request.uri.endsWith("/ignore"))
        return send_ignore(request.uri);
    else if (request.uri.startsWith("/feedback"))
        return send_feedback(request.uri, request.form);
});

function auth()
{
    return new Promise((resolve, reject) => {
        fetch(`${SERVER_IP}/ping`)
        .then(res => resolve(res.status));
    });
}

function get_missing()
{
    return new Promise((resolve, reject) => {
        fetch(`${SERVER_IP}/feedback/missing`)
        .then(res => resolve(res.json()));
    });
}

function get_details(uri)
{
    return new Promise((resolve, reject) => {
        fetch(`${SERVER_IP}${uri}`)
        .then(res => resolve(res.json()));
    });
}

function send_ignore(uri)
{
    return new Promise((resolve, reject) => {
        fetch(`${SERVER_IP}${uri}`, {
            method: "POST",
            body: JSON.stringify({ }),
            headers: {
                "Content-type": "application/json; charset=UTF-8"
            }
        })
        .then(resolve(null));
    });
}

function send_feedback(uri, data)
{
    return new Promise((resolve, reject) => {
        fetch(`${SERVER_IP}${uri}`, {
            method: "POST",
            body: JSON.stringify(data),
            headers: {
                "Content-type": "application/json; charset=UTF-8"
            }
        })
        .then(resolve(null)); //need to return a promise so the content_script knows when the background script is done
    });
}
