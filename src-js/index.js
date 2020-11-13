console.log("bullet hell v0.4.0");

import {App} from "./world.js";


window.addEventListener("load", function () {
  let el = document.getElementById("game");
  let app = new App(el);
  app.draw();
});

var zlib = require("zlib");

// Get elapsed time
var elapsed = (function() {
  var last = +new Date();
  return function () {
    var now = new Date();
    var dur = now-last;
    last = now; return dur; };
}());

// Send message to div
function message(id, s) {
  var el = document.getElementById(id);
  el.innerHTML = s;
  el.scrollTo(0, el.scrollHeight);
}

// Return game state from a run-length encoded format
function from_runlength (b) {
  var res = b.split(":");

  var dims = res.slice(0, 2);
  dims[0] = parseInt(dims[0]);
  dims[1] = parseInt(dims[1]);

  var grid = [];

  res.slice(2).forEach(function (k) {
    var [run, val] = k.split(",");
    val = val == "o" ? 1 : 0;

    run = parseInt(run);
    var chunk = new Array(run);
    chunk.fill(val);
    grid = grid.concat(chunk);
  });

  return {
    "dims": dims,
    "grid": grid,
  };
}

// Global game state
let GAME_STATE = undefined;
let CONNECTED = false;


// SETUP
var socket = new WebSocket("ws://" + location.host + "/ws");
var INTERVAL;

socket.binaryType = "arraybuffer";

socket.addEventListener("open", function (ev) {


  CONNECTED = true;

  (function send() {
    // console.log("ELAPSED:", elapsed(), "SIZE: ", ev.data.length);
    var rand = "*";
    setTimeout(function () {
      socket.send(rand);
    }, 0);

    message("sent", rand);

    if (CONNECTED) {
      setTimeout(send, 33);
    }
  }());


});

socket.addEventListener("close", function (ev) {
  CONNECTED = false;
  clearInterval(INTERVAL);
});

socket.addEventListener("error", function (ev) {
  clearInterval(INTERVAL);
});

socket.addEventListener("message", function (ev) {
  var b = Buffer.from(ev.data);
  var c = zlib.gunzipSync(b);
  var r = from_runlength(c.toString());
  // console.log("elapsed =>", elapsed());
  GAME_STATE = r;
});


(function loop() {
  // draw(GAME_STATE);
  requestAnimationFrame(loop);
}());



function draw() {

  console.log(">>", GAME_STATE);

  if (!GAME_STATE) {
    return;
  }

  var el = document.getElementById("game");
  var table = document.createElement("table");

  var h = GAME_STATE.dims[0];
  var w = GAME_STATE.dims[1];

  function pos(i, j) {
    return GAME_STATE.grid[h*i + j];
  }

  for (var i=0; i < GAME_STATE.dims[0]; i++) {

    var tr = document.createElement("tr");
    table.appendChild(tr);

    for (var j=0; j < GAME_STATE.dims[1]; j++) {
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
