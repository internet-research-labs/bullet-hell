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
    this.players = new Map();

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

  updatePlayer(obj) {
    let {id, pos, dir} = obj;
    let player = this.players.has(id) ? this.players.get(id) : false;

    if (!player) {
      let mesh = MeshBuilder.CreateBox(
        "PLAYER:"+id,
        {width: 3, height: 3},
        this.scene,
      );

      player = {
        mesh: mesh,
        meta: obj,
      };

      player.mesh.material = this.shipMaterial;
      this.players.set(id, player);
    }

    player.mesh.position.x = pos.x;
    player.mesh.position.y = 6.0;
    player.mesh.position.z = pos.y;

    player.meta = obj;
  }

  update(obj) {
    this.updatePlayer(obj);
  }

  tick() {

    // Replace this with "tick"
    this.players.forEach(v => {
      v.mesh.position.x += v.meta.dir.x;
      v.mesh.position.y += 0.0;
      v.mesh.position.z += v.meta.dir.y;
    });
  }

  draw() {
    this.scene.render();
  }
}
