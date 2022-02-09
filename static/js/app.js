const notesBody = document.getElementById("notes");
const notesData = document.getElementById("inputnote");
const noteError = document.getElementById("newerror");
const noteErrorMessage = document.getElementById("newerrormsg");
const noteCount = document.getElementById("count");
const loader = document.getElementById("loader");
var lastNote = notesBody.getElementsByClassName("isnote")[0];
var currentPage = 1;
var pagePointerCount = document.getElementsByClassName("page-item").length - 2;
var lastPagePointer = document.getElementsByClassName("page-item")[1];

document.onkeyup = function (e) {
  if (e.ctrlKey && e.key === "Enter" && document.activeElement === notesData) {
    submitAndUpdate();
  }
};

function send(url, type, data, oncomplete, onfail) {
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
  XHR.open(type, url);
  XHR.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
  XHR.send(encodedData);
  XHR.onreadystatechange = function () {
    if (XHR.readyState == XMLHttpRequest.DONE) {
      if (XHR.status === 201) {
        oncomplete(XHR);
      } else {
        onfail(XHR);
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
    send(
      "/create/note",
      "POST",
      { note: note },
      function (XHR) {
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
      },
      function () {
        loader.hidden = true;
        noteErrorMessage.innerText = "Failed to submit new note";
        noteError.hidden = false;
      }
    );
  }
  notesData.focus();
}

function getPage(_id) {
  console.log("Loading page: ", currentPage);
}

function loadPage(elem) {
  lastPagePointer.classList.remove("active");
  lastPagePointer = elem;
  elem.classList.add("active");
  currentPage = parseInt(elem.innerText);
  getPage(currentPage);
}

function loadPagePrev() {
  if (currentPage != 1) {
    // ignore click when on first already
    lastPagePointer.classList.remove("active");
    var newPointer =
      document.getElementsByClassName("page-item")[currentPage - 1];
    newPointer.classList.add("active");
    lastPagePointer = newPointer;
    currentPage -= 1;
    getPage(currentPage);
  }
}

function loadPageNext() {
  if (currentPage != pagePointerCount) {
    lastPagePointer.classList.remove("active");
    var newPointer =
      document.getElementsByClassName("page-item")[currentPage + 1];
    newPointer.classList.add("active");
    lastPagePointer = newPointer;
    currentPage += 1;
    getPage(currentPage);
  }
}
