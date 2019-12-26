export class Asteroid {
  constructor(public x: number, public y: number) {}

  relativeSlopeTo(other: Asteroid) {
    const rise = this.y - other.y;
    const run = other.x - this.x;
    return new Slope(rise, run);
  }
  distanceTo(other) {
    const rise = this.y - other.y;
    const run = other.x - this.x;
    return Slope.distance(rise, run);
  }
  location() {
    return `${this.x},${this.y}`;
  }
}

export class Slope {
  rise: number;
  run: number;
  degrees: number;
  radius: number;
  constructor(rise: number, run: number) {
    [this.rise, this.run] = this.reduceSlope(rise, run);
    this.degrees = this.to_degrees(rise, run);
    this.radius = Slope.distance(rise, run);
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
  to_degrees(rise: number, run: number) {
    return ((Math.atan2(rise, run) * 180) / Math.PI + 360) % 360;
  }
  static distance(rise: number, run: number) {
    return Math.sqrt(rise * rise + run * run);
  }
  to_string() {
    return `${this.rise}/${this.run}`;
  }
}
