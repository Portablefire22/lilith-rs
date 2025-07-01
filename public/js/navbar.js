
const navbarItems = document.getElementsByClassName("navbar-item"); 
let index = navbarItems.length - 1;

function navbarLeft() {
  navbarItems[index].id = "";
  index--;
  if (index < 0) {
    index = navbarItems.length - 1;
  }
  setCookie("navbar", index);
  navbarItems[index].id = "navbar-active";
}

function navbarRight() {  
  navbarItems[index].id = "";
  index++;
  if (index > navbarItems.length - 1) {
    index = 0;
  }
  console.log(index);
  setCookie("navbar", index);
  navbarItems[index].id = "navbar-active";
}


function loadNavbar() {
  let cookie = getCookie("navbar");
  if (cookie === undefined) {
    setCookie("navbar", navbarItems.length - 1);
    cookie = navbarItems.length - 1;
  }
  index = cookie;
  navbarItems[index].id = "navbar-active";
}

loadNavbar();
