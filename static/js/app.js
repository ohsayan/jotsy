const notesBody = document.getElementById("notes");
const notesData = document.getElementById("inputnote");
const noteError = document.getElementById("newerror");
const noteErrorMessage = document.getElementById("newerrormsg");

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
        notesBody.innerHTML = XHR.responseText + notesBody.innerHTML;
      } else {
        noteError.hidden = false;
        noteErrorMessage.innerText = "Failed to submit new note";
      }
    }
  };
}

function submitAndUpdate() {
  var note = notesData.value;
  if (note.length == 0) {
    noteError.hidden = false;
    noteErrorMessage.innerText = "Note cannot be empty!";
  } else {
    // hide any previous error message
    noteError.hidden = true;
    send({ note: note });
  }
}
