var socket = new WebSocket('ws://localhost:8080/ws');

function message(id, s) {
  var el = document.getElementById(id);
  el.innerHTML += s + "<br>";
}

socket.addEventListener("open", function (ev) {
  setInterval(function() {
    var rand = "" + Math.random();
    socket.send(rand);
    message("sent", rand);
  }, 1000);
});

socket.addEventListener("message", function (ev) {
  message("received", ev.data);
});
