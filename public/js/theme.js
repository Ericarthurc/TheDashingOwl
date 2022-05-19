let theme = localStorage.getItem("theme");
let themeSwitcherInput = document.querySelector("#themeSwitch");

if (theme === "light") {
  console.log("light");
  document.documentElement.setAttribute("data-theme", "light");
  themeSwitcherInput.checked = false;
} else {
  console.log("dark");
  document.documentElement.setAttribute("data-theme", "dark");
  themeSwitcherInput.checked = true;
}

themeSwitcherInput.addEventListener("change", (event) => {
  if (event.target.checked) {
    localStorage.setItem("theme", "dark");
    document.documentElement.setAttribute("data-theme", "dark");
  } else {
    localStorage.setItem("theme", "light");
    document.documentElement.setAttribute("data-theme", "light");
  }
});
