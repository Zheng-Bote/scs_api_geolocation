document.getElementById("msgbox").style.display = "none";

const form = document.getElementById("frm_add_provider");
form.addEventListener("submit", function (event) {
  // First, prevent the default form submission behavior
  event.preventDefault();

  document.getElementById("msgbox").style.display = "none";

  // Collect the form data
  let name = document.getElementById("name");
  let description = document.getElementById("description");
  let api_key = document.getElementById("api_key");
  let counter_limit = document.getElementById("counter_limit");
  const obj = {
    name: name.value,
    description: description.value,
    api_key: api_key.value,
    counter_limit: counter_limit.value,
    counter: "0",
    date_time: 0,
  };

  const myJSON = JSON.stringify(obj);

  send_frm_add_provider(myJSON);
});

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
