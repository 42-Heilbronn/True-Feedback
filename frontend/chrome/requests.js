const SERVER_IP = "https://dev01.playground.extension.42heilbronn.de/api";

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    if (request == "auth")
        return auth(sendResponse);
    else if (request == "miss")
        return get_missing(sendResponse);
    else if (request.uri.endsWith("info"))
        return get_details(request.uri, sendResponse);
    else if (request.uri.startsWith("/feedback"))
        return send_feedback(request.uri, request.form, sendResponse);
    console.log(request);
});

function auth(sendResponse)
{
    fetch(`${SERVER_IP}/ping`)
    .then(res => sendResponse(res.status));
    return true;
}

function get_missing(sendResponse)
{
    fetch(`${SERVER_IP}/feedback/missing`)
    .then(res => res.json()).then(json => sendResponse(json));
    return true;
}

function get_details(uri, sendResponse)
{
    fetch(`${SERVER_IP}${uri}`)
    .then(res => res.json()).then(json => sendResponse(json));
    return true;
}

function send_feedback(uri, data, sendResponse)
{
    fetch(`${SERVER_IP}${uri}`, {
        method: "POST",
        body: JSON.stringify(data),
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    })
    .then(sendResponse(null));
    return true;
}
