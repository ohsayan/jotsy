const notesBody = document.getElementById("notes");
const notesData = document.getElementById("inputnote");
const noteError = document.getElementById("newerror");
const noteErrorMessage = document.getElementById("newerrormsg");
const noteCount = document.getElementById("count");
var lastNote = notesBody.getElementsByClassName("isnote")[0];

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
  XHR.open("POST", "/createnote");
  XHR.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
  XHR.send(encodedData);
  var ret;
  XHR.onreadystatechange = function () {
    if (XHR.readyState == XMLHttpRequest.DONE) {
      if (XHR.status === 201) {
        var transientBody = document.getElementById("notes");
        var element = document.createElement("span");
        element.innerHTML = XHR.responseText;
        transientBody.insertBefore(element, lastNote);
        lastNote = element;
        if (document.getElementById("nonewnotes") != null) {
          document.getElementById("nonewnotes").remove();
        }
        var n = parseInt(noteCount.textContent);
        n += 1;
        noteCount.textContent = n.toString();
        notesData.value = "";
      } else {
        noteError.hidden = false;
        noteErrorMessage.innerText = "Failed to submit new note";
      }
    }
  };
}

function submitAndUpdate() {
  var note = notesData.value;
  if (note.length === 0) {
    noteError.hidden = false;
    noteErrorMessage.innerText = "Note cannot be empty!";
  } else {
    // hide any previous error message
    noteError.hidden = true;
    send({ note: note });
  }
}
