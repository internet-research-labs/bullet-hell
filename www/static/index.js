var socket = new WebSocket('ws://localhost:8080/ws');

function message(s) {
  var el = document.getElementById("what");
  el.innerHTML += s + "<br>";
}

socket.addEventListener("open", function (ev) {
  setInterval(function() {
    // var rand = "" + Math.random();
    // socket.send(rand);
    // message(rand);
  }, 1000);
});

socket.addEventListener("message", function (ev) {
  console.log("Received2:", ev.data);
  message(ev.data);
});
