interface Position {
  x: number;
  y: number;
  z: number;
}
interface Velocity {
  x: number;
  y: number;
  z: number;
}

export class System {
  moons: Array<Moon>;
  init: string;
  steps: number;
  constructor(moons: Array<Moon>) {
    this.moons = moons;
    this.init = this.hash();
    this.steps = 0;
  }
  steps_til_repeat() {
    let x_rep = true;
    let y_rep = true;
    let z_rep = true;
    const cycles = { x: 0, y: 0, z: 0 };
    while (x_rep || y_rep || z_rep) {
      this.step();
      const state = this.moons.map(m => m.isInit());

      const isX = state.filter(s => s.includes("x")).length === state.length;
      const isY = state.filter(s => s.includes("y")).length === state.length;
      const isZ = state.filter(s => s.includes("z")).length === state.length;

      if (x_rep) {
        cycles["x"] += 1;
        if (isX) {
          x_rep = false;
        }
      }
      if (y_rep) {
        cycles["y"] += 1;
        if (isY) {
          y_rep = false;
        }
      }
      if (z_rep) {
        cycles["z"] += 1;
        if (isZ) {
          z_rep = false;
        }
      }
      if (!x_rep && !y_rep && !z_rep) {
        return lcm(cycles.x, cycles.y, cycles.z);
      }
    }
  }
  hash() {
    return this.moons.map(m => m.hash()).join("|");
  }
  total_energy() {
    return this.moons.map(m => m.energy()).reduce(sum);
  }
  step(times: number = 1) {
    for (let i = 0; i < times; i++) {
      this.steps += 1;
      this.updateVelocities();
      this.updatePositions();
    }
  }
  updateVelocities() {
    for (let i = 0; i < this.moons.length; i++) {
      const a = this.moons[i];
      for (let j = i; j < this.moons.length; j++) {
        if (i == j) {
          continue;
        }
        const b = this.moons[j];
        System.compose(a, b);
      }
    }
  }
  updatePositions() {
    this.moons.forEach(m => m.step());
  }

  static compose(a: Moon, b: Moon) {
    ["x", "y", "z"].forEach(k => {
      if (a.position[k] > b.position[k]) {
        System.increment(b, a, k);
      } else if (b.position[k] > a.position[k]) {
        System.increment(a, b, k);
      } else {
        // Noop
      }
    });
  }
  static increment(up: Moon, down: Moon, k: string) {
    up.velocity[k]++;
    down.velocity[k]--;
  }
}

export class Moon {
  init_pos: Position;
  init_vel: Velocity;
  constructor(
    public position: Position,
    public velocity: Velocity = { x: 0, y: 0, z: 0 }
  ) {
    this.position = position;
    this.velocity = velocity;
    this.init_pos = { ...position };
    this.init_vel = { ...velocity };
  }
  static from(str: string) {
    str = str.replace(/<|>/, "");
    const [x, y, z] = str.split(",").map(c => {
      return parseInt(c.split("=")[1].trim());
    });
    return new Moon({ x, y, z });
  }
  isInit() {
    const matches = ["x", "y", "z"].filter(e => {
      const pmatch = this.position[e] === this.init_pos[e];
      const vmatch = this.velocity[e] === this.init_vel[e];
      return pmatch && vmatch;
    });
    return matches;
  }
  step() {
    this.position.x += this.velocity.x;
    this.position.y += this.velocity.y;
    this.position.z += this.velocity.z;
  }
  energy() {
    const pot = Object.values(this.position)
      .map(Math.abs)
      .reduce(sum);
    const kin = Object.values(this.velocity)
      .map(Math.abs)
      .reduce(sum);

    return pot * kin;
  }
  hash() {
    const p = this.position;
    const v = this.velocity;
    return `p${p.x}.${p.y}.${p.z}v${v.x}.${v.y}.${v.z}`;
  }
}

const sum = (a: number, b: number) => a + b;
const lcm = (a: number, b: number, c: number) => {
  const ab = (a * b) / gcd(a, b);
  return (ab * c) / gcd(ab, c);
};
const gcd = (a, b) => {
  if (a === 0) {
    return b;
  }
  return gcd(b % a, a);
};
