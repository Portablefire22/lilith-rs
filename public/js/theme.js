
function setTheme(themeName) {
  setCookie("theme", themeName, 30);
  document.getElementById('theme-css').setAttribute('href', '/public/css/theme-' + themeName + '.css');
}

function getCookie(name) {
  const regex = new RegExp(`(^| )${name}=([^;]+)`)
  const match = document.cookie.match(regex)
  if (match) {
    return match[2]
  }
}

function setCookie(name,value,days) {
    var expires = "";
    if (days) {
        var date = new Date();
        date.setTime(date.getTime() + (days*24*60*60*1000));
        expires = "; expires=" + date.toUTCString();
    }
    document.cookie = name + "=" + (value)  + expires + "; path=/";
}

function loadTheme() {
  console.log(document.cookie);
  let cookie = getCookie("theme");
  if (cookie === undefined) {
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
      setCookie("theme", "dark", 30);
      cookie = "dark";
    } else {
      setCookie("theme", "light", 30);
      cookie = "light";
    }
  }
  setTheme(cookie);
}

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', event => {
    const newColorScheme = event.matches ? "dark" : "light";
    setTheme(newColorScheme);
});

loadTheme();
