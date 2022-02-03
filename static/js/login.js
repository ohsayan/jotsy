var pwd = document.getElementById("pass");
var vpwd = document.getElementById("vpass");
var form = document.getElementById("signup-form");

function verifyPassword() {
  var pass = pwd.value;
  var vpass = vpwd.value;
  if (pass == vpass) {
    form.submit();
    console.log("yes");
  } else {
    alert("The passwords don't match");
  }
}
