import { Vector } from "./vector"
import { Point } from "./point"
import { Instruction } from "./instruction"

export class Wire {
    path: Array<Vector>
    origin: Point

    constructor(instructions: Array<Instruction>, origin: Point = new Point(0,0)) {
        this.path = this.drawPath(instructions, origin.move() )
        this.origin = origin
    }

    drawPath(instructions: Array<Instruction>, origin: Point){
        let ptr = origin.move()
        return instructions.map( i => {
            const vec = new Vector(i, ptr.move())
            ptr = vec.end()
            return vec
        })
    }

    static from_string(str: string){
        const instructions = str.trim().split(",").map( i => Instruction.from_str(i) )
        return new Wire(instructions)
    }
}