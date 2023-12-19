function send_frm_add_provider(jsondata) {
  fetch("/create", {
    body: jsondata,
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
  })
    .then(function (response) {
      return response.json();
    })
    .then(function (json) {
      console.log("return: ", json);
      if (json.error) {
        console.error(json.error.code);
        console.error(json.error.reason);
        msgBox(json.error.reason, false);
      } else {
        msgBox("Entity added", true);
        document.getElementById("frm_add_provider").reset();
      }
    })
    .catch(function (error) {
      console.log("Request failed", error);
    });
}

function msgBox(text, type) {
  const msgBox = document.getElementById("msgbox");
  if (msgBox.style.display === "none") {
    msgBox.style.display = "block";
    if (type === false) {
      msgBox.setAttribute("Class", "red");
    } else {
      msgBox.setAttribute("Class", "green");
    }
    msgBox.innerHTML = "<p>" + text + "</p>";
  } else {
    msgBox.style.display = "none";
    msgBox.innerText = "";
  }
}
