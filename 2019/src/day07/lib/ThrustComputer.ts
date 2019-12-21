import { Amplifier } from "./Amplifier";

export class ThrustComputer {
  amps: Array<Amplifier>;
  config: Array<Array<number>>;

  constructor(amps: Array<Amplifier>, config: Array<number>) {
    if (amps.length !== config.length) {
      throw "Incompatible config";
    }
    this.amps = amps;
    this.config = config.map(c => [c]);
  }

  computeThrust() {
    let out = 0;
    for (let i = 0; i < this.amps.length; i++) {
      const amp = this.amps[i];
      const state = this.config[i];
      out = amp.compute([...state, out]);
    }
    return out;
  }

  computeThrustWithFeedback() {
    let out = 0;
    let i = 0;
    let initState = true;
    while (true) {
      const amp = this.amps[i];
      let input = [...this.config[i]];
      if (initState) {
        input = [...input, 0];
        initState = false;
      } else if (this.config[i]) {
        input = [...input, out];
      }
      this.config[i] = input;
      const res = amp.compute(this.config[i]);
      if (res) {
        out = res;
      } else {
        return out;
      }
      i++;
      i = i % this.amps.length;
    }
  }
}
