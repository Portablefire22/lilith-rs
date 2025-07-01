
function setTheme(themeName) {
  document.getElementById('theme-css').setAttribute('href', '/public/css/theme-' + themeName + '.css');
}

function getCookie(name) {
  const regex = new RegExp(`(^| )${name}=([^;]+)`)
  const match = document.cookie.match(regex)
  if (match) {
    return match[2]
  }
}

function loadTheme() {
  console.log(document.cookie);
  let cookie = getCookie("theme");
  if (cookie === undefined) {
    cookie = "dark";
  }
  setTheme(cookie);
}


loadTheme();
