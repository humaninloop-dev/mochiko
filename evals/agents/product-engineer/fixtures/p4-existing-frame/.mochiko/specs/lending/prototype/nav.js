// Shared nav — injected into <header data-nav> on every screen.
const links = [
  ["index.html", "Home"],
  ["screens/scr-001-catalogue.html", "Catalogue"],
  ["screens/scr-003-my-loans.html", "My loans"],
  ["screens/scr-004-reservations.html", "Reservations (coming soon)"],
];
const base = location.pathname.includes("/screens/") ? "../" : "./";
document.querySelector("[data-nav]").innerHTML =
  links.map(([href, label]) => `<a href="${base}${href}">${label}</a>`).join(" · ");
