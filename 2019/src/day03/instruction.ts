import { Point } from "./point"

export class Instruction {
    direction: string
    distance: number

    constructor(direction: string, distance: number) {
        this.direction = direction
        this.distance = distance
    }

    static from_str(str: string, origin: Point = new Point(0,0)){
        const dir  = str[0]
        const dist = str.slice(1)

        return new Instruction(dir, parseInt(dist))
    }
}