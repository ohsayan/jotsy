const notesBody = document.getElementById("notes");
const notesData = document.getElementById("inputnote");
const noteError = document.getElementById("newerror");
const noteErrorMessage = document.getElementById("newerrormsg");
const noteCount = document.getElementById("count");
var lastNote = notesBody.getElementsByClassName("isnote")[0];
const loader = document.getElementById("loader");

document.onkeyup = function (e) {
  if (e.ctrlKey && e.key === "Enter" && document.activeElement === notesData) {
    submitAndUpdate();
  }
};

function send(data) {
  const XHR = new XMLHttpRequest();
  var encodedData = "",
    encodedDataPairs = [],
    name;
  for (name in data) {
    encodedDataPairs.push(
      encodeURIComponent(name) + "=" + encodeURIComponent(data[name])
    );
  }
  encodedData = encodedDataPairs.join("&").replace(/%20/g, "+");
  XHR.open("POST", "/create/note");
  XHR.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
  XHR.send(encodedData);
  var ret;
  XHR.onreadystatechange = function () {
    if (XHR.readyState == XMLHttpRequest.DONE) {
      if (XHR.status === 201) {
        var element = document.createElement("span");
        element.innerHTML = String(XHR.responseText);
        notesBody.insertBefore(element, lastNote);
        lastNote = element;
        if (document.getElementById("nonewnotes") != null) {
          document.getElementById("nonewnotes").remove();
        }
        loader.hidden = true;
        var n = parseInt(noteCount.textContent);
        n += 1;
        noteCount.textContent = n.toString();
        notesData.innerText = "";
      } else {
        noteError.hidden = false;
        loader.hidden = true;
        noteErrorMessage.innerText = "Failed to submit new note";
      }
    }
  };
}

function submitAndUpdate() {
  var note = notesData.innerText;
  if (note.length === 0) {
    noteError.hidden = false;
    noteErrorMessage.innerText = "Note cannot be empty!";
  } else {
    // hide any previous error message
    noteError.hidden = true;
    loader.hidden = false;
    send({ note: note });
  }
  notesData.focus();
}
