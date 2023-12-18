window.onload = start;

function start() {
  setFooter();

  document.getElementById("btn_top_new").addEventListener("click", function () {
    gotoTop();
  });
  const observer = new IntersectionObserver(callback, options);
  const seetop = document.getElementById("body");
  observer.observe(seetop);
}

function setFooter() {
  const VERSION = "v00.01.00";
  const CREATED = "2023";
  const DATE = new Date();
  const FULLYEAR = DATE.getFullYear();
  let years = "";

  CREATED == FULLYEAR ? (years = CREATED) : (years = CREATED + "-" + FULLYEAR);

  document.getElementById(
    "footer"
  ).innerHTML = `&copy; ZHENG Robert ${years}, ${VERSION}`;
  document.getElementById("footer").addEventListener("click", function () {
    window.location.href = "#body";
  });
}

function gotoTop() {
  window.scrollTo({ left: 0, top: 0, behavior: "smooth" });
}

const callback = (entries) => {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      // 'entry.target' is the DOM element
      //console.log(`${entry.target.id} is visible.`);

      //const intersecting = entry.isIntersecting;
      //entry.target.style.backgroundColor = intersecting ? "blue" : "orange";
    } else {
      //console.log(`${entry.target.id} is not visible.`);
    }
    if (entry.target.id === "body") {
      const intersecting = entry.isIntersecting;
      const btnTop = document.getElementById("btn_top_new");
      btnTop.style.display = intersecting ? "none" : "block";
    }
  });
};
const options = {
  threshold: 1.0,
};
