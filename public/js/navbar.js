
const navbarItems = document.getElementsByClassName("navbar-item"); 
let index = navbarItems.length - 1;

function navbarLeft() {
  navbarItems[index].id = "";
  index--;
  if (index < 0) {
    index = navbarItems.length - 1;
  }
  navbarItems[index].id = "navbar-active";
}

function navbarRight() {  
  navbarItems[index].id = "";
  index++;
  if (index > navbarItems.length - 1) {
    index = 0;
  }
  navbarItems[index].id = "navbar-active";
}
