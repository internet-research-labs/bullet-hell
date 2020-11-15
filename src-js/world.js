import {
  Engine,
  Scene,
  ArcRotateCamera,
  Vector3,
  HemisphericLight,
  Mesh,
  MeshBuilder,
  TransformNode,
} from "@babylonjs/core";

import * as BABYLON from "@babylonjs/core";

export class App {
  constructor(el) {

    // List of players
    this.players = {};

    let engine = new Engine(el, true);
    this.scene = new Scene(engine);
    let camera = new ArcRotateCamera(
      "camera",
      0,
      0,
      0,
      new Vector3(0, 0, 0),
      this.scene,
    );

    camera.setPosition(new Vector3(0, 50, -250));

    let light = new HemisphericLight(
      "light",
      new Vector3(0, 1, 0),
      this.scene,
    );

    const plane = MeshBuilder.CreateGround(
      "plane",
      {width: 200, height: 200},
      this.scene,
    );

    this.shipMaterial = new BABYLON.StandardMaterial("ship", this.scene);
    this.shipMaterial.disableLighting = true;
    this.shipMaterial.emissiveColor = new BABYLON.Color3(0, 1, 1);
  }

  updatePlayer({id, pos}) {
    let player = this.players.hasOwnProperty(id) ? this.players[id] : false;

    if (!player) {
      player = MeshBuilder.CreateBox(
        "PLAYER:"+id,
        {width: 3, height: 3},
        this.scene,
      );
      player.material = this.shipMaterial;
      player = this.players[id] = player;
    }

    player.position.x = pos.x;
    player.position.y = 6;
    player.position.z = pos.y;
  }

  update(obj) {
    this.updatePlayer(obj);
  }

  draw() {
    this.scene.render();
  }
}
