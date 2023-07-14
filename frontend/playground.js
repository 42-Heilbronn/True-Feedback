browser.cookies.get({url: "https://dev01.playground.extension.42heilbronn.de/*", name: "id"}).then(cookie => {
    browser.storage.local.clear();
    browser.storage.local.set({cookie})
});