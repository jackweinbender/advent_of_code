export class Asteroid {
  constructor(public x: number, public y: number) {}

  relativeSlopeTo(other: Asteroid) {
    const rise = this.y - other.y;
    const run = other.x - this.x;
    return new Slope(rise, run).to_string();
  }

  location() {
    return `${this.x}-${this.y}`;
  }
}

export class Slope {
  constructor(private rise: number, private run: number) {
    [this.rise, this.run] = this.reduceSlope(rise, run);
  }
  reduceSlope(rise, run) {
    if (rise === 0) {
      if (run > 0) {
        return [0, 1];
      } else {
        return [0, -1];
      }
    }
    if (run === 0) {
      if (rise > 0) {
        return [1, "inf"];
      } else {
        return [-1, "inf"];
      }
    }
    for (let i = Math.abs(rise); i > 0; i--) {
      if (rise % i == 0 && run % i == 0) {
        rise = rise / i;
        run = run / i;
        break;
      }
    }
    return [rise, run];
  }
  to_string() {
    return `${this.rise}/${this.run}`;
  }
}
