browser.runtime.onMessage.addListener((request, sender) => {
    return new Promise((resolve, reject) => {
        fetch(`https://dev01.playground.extension.42heilbronn.de/api/ping`)
        .then(res => resolve(res.status));
    });
  });