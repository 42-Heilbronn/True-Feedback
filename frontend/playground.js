function logCookies(cookies) {
    for (const cookie of cookies) {
      console.log(cookie.value);
    }
  }

browser.cookies.getAll({name: "id",}).then(logCookies);