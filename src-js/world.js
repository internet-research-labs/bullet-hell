import {
  Engine,
  Scene,
  ArcRotateCamera,
  Vector3,
  HemisphericLight,
  Mesh,
  MeshBuilder,
} from "@babylonjs/core";

export class App {
  constructor(el) {
    let engine = new Engine(el, true);
    this.scene = new Scene(engine);
    let camera = new ArcRotateCamera(
      "Camera",
      -Math.PI/1.5,
      +Math.PI/3,
      10,
      new Vector3(0, 0, 0),
      this.scene,
    );

    let light = new HemisphericLight(
      "light",
      new Vector3(0, 1, 0),
      this.scene,
    );

    let box = MeshBuilder.CreateBox(
      "box",
      {},
      this.scene,
    );

    box.position.y = 0.5;

    const plane = MeshBuilder.CreateGround(
      "plane",
      {width: 10, height: 10},
      this.scene,
    );
  }

  update() {
  }

  draw() {
    this.scene.render();
  }
}
