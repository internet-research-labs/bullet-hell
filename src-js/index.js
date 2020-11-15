console.log("bullet hell v0.4.0");

import {App} from "./world.js";

var APP = undefined;

window.addEventListener("load", function () {
  let el = document.getElementById("game");
  APP = new App(el);
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
  var d = c.toString();
  let updates = get_state(d);

  updates.forEach((v, _) => {
    APP.update(v)
  });
});

function process(blob) {
  let pieces = blob.split(",");
  if (pieces.length != 5) {
    return undefined;
  }
  return {
    id: pieces[0],
    pos: {x: pieces[1], y: pieces[2]},
    dir: {x: pieces[3], y: pieces[4]},
  };
}

// Return the state
function get_state(s) {
  let res = [];
  s.split(":").forEach((v, _) => {
    let up = process(v);
    res.push(up);
  });
  return res;
}


(function loop() {
  if (!!APP) {
    APP.draw();
  }
  requestAnimationFrame(loop);
}());
