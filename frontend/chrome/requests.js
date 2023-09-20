const SERVER_IP = "https://dev01.playground.extension.42heilbronn.de/api";

//listens for a message from the content script and sends a request to the server depending on the message
//sendResponse to send an answer back to the content script
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    if (request == "auth")
        return auth(sendResponse);
    else if (request == "miss")
        return get_missing(sendResponse);
    else if (request.uri.endsWith("info"))
        return get_details(request.uri, sendResponse);
    else if (request.uri.endsWith("/ignore"))
        return send_ignore(request.uri, sendResponse);
    else if (request.uri.startsWith("/feedback"))
        return send_feedback(request.uri, request.form, sendResponse);
    console.log(request);
});

function auth(sendResponse)
{
    fetch(`${SERVER_IP}/ping`)
    .then(res => sendResponse(res.status));
    return true; //if you return true, chrome knows that this function is running async
}

function get_missing(sendResponse)
{
    fetch(`${SERVER_IP}/feedback/missing`)
    .then(res => res.json()).then(json => sendResponse(json)); //.json() is a promise, so it awaits for it to finish and returns the result afterwards
    return true;
}

function get_details(uri, sendResponse)
{
    fetch(`${SERVER_IP}${uri}`)
    .then(res => res.json()).then(json => sendResponse(json));
    return true;
}

function send_ignore(uri, sendResponse)
{
    fetch(`${SERVER_IP}${uri}`, {
            method: "POST",
            body: JSON.stringify({ }),
            headers: {
                "Content-type": "application/json; charset=UTF-8"
            }
        })
        .then(sendResponse(null));
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
