import { fewestZeros, numLayers } from "./lib";
type Layer = Array<number>;

export class Image {
  layers: Array<Layer>;
  x: number;
  y: number;
  layerSize: number;
  constructor(layers: Array<Layer>, x: number, y: number) {
    this.layers = layers;
    this.x = x;
    this.y = y;
  }

  static from_input(input: Array<number>, x: number, y: number) {
    const layerSize = x * y;

    let layers = new Array(numLayers(input, layerSize));

    for (let i = 0; i < input.length; i++) {
      const l = Math.floor(i / layerSize);
      if (!layers[l]) {
        layers[l] = [];
      }
      layers[l].push(input[i]);
    }

    return new Image(layers, x, y);
  }

  twosByOnes() {
    const fewest = this.layers.reduce(fewestZeros);
    const twos = fewest.filter(e => e === 2).length;
    const ones = fewest.filter(e => e === 1).length;
    return twos * ones;
  }
  getVisible() {
    return this.layers.reduce((curr, next) => {
      for (let i = 0; i < curr.length; i++) {
        if (curr[i] === 2) {
          curr[i] = next[i];
        }
      }
      return curr;
    });
  }
  display() {
    const visible = this.getVisible().map(p => (p === 0 ? "." : "#"));
    for (let i = 0; i < this.y; i++) {
      const start = i * this.x;
      const stripe = visible.slice(start, start + this.x);
      console.log(stripe.join(""));
    }
  }
}
