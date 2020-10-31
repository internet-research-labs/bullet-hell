var socket = new WebSocket('ws://localhost:8080/ws');

socket.addEventListener("open", function (ev) {
  var rand = "" + Math.random();
  console.log("Well-connected. Sending:", rand);
  socket.send(rand);
});

socket.addEventListener("message", function (ev) {
  console.log("Received:", ev.data);
});
