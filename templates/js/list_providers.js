fetch("/providers")
  .then(function (response) {
    return response.json();
  })
  .then(function (json) {
    const obj = json;

    const article = document.getElementById("all_providers");
    for (i in obj) {
      let section = document.createElement("section");

      let hgroup = document.createElement("hgroup");
      let h3 = document.createElement("h3");
      h3.innerText = obj[i].name;
      hgroup.appendChild(h3);
      let p = document.createElement("p");
      p.innerText = obj[i].description;
      hgroup.appendChild(p);
      section.appendChild(hgroup);

      let details = document.createElement("details");
      let summary = document.createElement("summary");
      summary.innerText = "Details";
      details.appendChild(summary);
      let details_p = document.createElement("p");
      details_p.innerHTML = "<table>";
      details_p.innerHTML = `<table><tr><td>API key:</td><td>${obj[i].api_key}</td></tr><tr><td>Payload limit:</td><td>${obj[i].counter_limit}</td></tr><tr><td>Counter:</td><td>${obj[i].counter}</td></tr></table>`;
      details.appendChild(details_p);
      section.appendChild(details);

      let btn = document.createElement("button");
      btn.setAttribute("id", obj[i].id);
      btn.innerText = "edit";
      section.appendChild(btn);

      article.appendChild(section);
    }
  });
