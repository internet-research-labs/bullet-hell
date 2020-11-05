
console.log("x_x");

var socket = new WebSocket("ws://" + location.host + "/ws");

var interval;

function message(id, s) {
  var el = document.getElementById(id);
  el.innerHTML += s + "<br>";
  el.scrollTo(0, el.scrollHeight);
}

socket.addEventListener("open", function (ev) {
  interval = setInterval(function() {
    var rand = "" + Math.random();
    socket.send(rand);
    message("sent", rand);
  }, 1000);
});

var elapsed = (function() {

  var last = +new Date();

  return function () {
    var now = new Date();
    var dur = now-last;
    last = now;
    return dur;
  };

}());

socket.addEventListener("message", function (ev) {
  console.log("elapsed:", elapsed());
  message("received", ev.data.substring(100, 140) + "...");

  var lis = ev.data.split(":");

  let o = {
    "dims": lis.slice(0, 2),
    "grid": lis.slice(2),
  };

  // var msg = JSON.parse(ev.data);
  render_table(o);
});

socket.addEventListener("close", function (ev) {
  clearInterval(interval);
});


function render_table(msg) {

  var el = document.getElementById("game");
  var table = document.createElement("table");

  var h = msg.dims[0];
  var w = msg.dims[1];

  function pos(i, j) {
    return msg.grid[h*i + j];
  }

  for (var i=0; i < msg.dims[0]; i++) {

    var tr = document.createElement("tr");
    table.appendChild(tr);

    for (var j=0; j < msg.dims[1]; j++) {
      var td = document.createElement("td");
      tr.appendChild(td);

      td.className = pos(i, j) == 1 ? "alive" : "dead";
    }
  }

  while (el.firstChild) {
    el.removeChild(el.firstChild);
  }

  el.appendChild(table);
}
