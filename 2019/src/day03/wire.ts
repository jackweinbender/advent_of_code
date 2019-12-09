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
        let start_length = 0;
        return instructions.map( i => {
            const vec = new Vector(i, ptr.move(), start_length)
            
            start_length += vec.instruction.distance
            ptr = vec.end()
            return vec
        })
    }
    distance_to_point(point: Point){
        const vec = this.path.find( v => {
            return v.contains_point(point)
        });
        return vec.start_length + vec.origin.distance(point)
    }

    static from_str(str: string){
        const instructions = str.trim().split(",").map( i => Instruction.from_str(i) )
        return new Wire(instructions)
    }
}