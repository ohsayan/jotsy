const pwd = document.getElementById("pass");
const vpwd = document.getElementById("vpass");
const form = document.getElementById("signup-form");

function verifyPassword() {
  var pass = pwd.value;
  var vpass = vpwd.value;
  if (pass == vpass) {
    form.submit();
  } else {
    alert("The passwords don't match");
  }
}
