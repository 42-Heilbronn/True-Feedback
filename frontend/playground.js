browser.cookies.get({url: "https://dev01.playground.extension.42heilbronn.de/*", name: "id"}).then(cookie => { //also add this to onMessage
    browser.storage.local.clear();
    browser.storage.local.set({cookie})
});

browser.runtime.onMessage.addListener(function(request, sender) { //check the requests and do every fetch request in here
    fetch(`https://dev01.playground.extension.42heilbronn.de/api/ping`).then(res => console.log(res))
});
   