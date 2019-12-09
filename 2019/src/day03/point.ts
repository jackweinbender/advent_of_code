import { Instruction } from "./instruction"

export class Point {
    x: number
    y: number

    constructor(x: number, y: number) {
        this.x = x
        this.y = y
    }
    distance(from: Point = new Point(0,0)){
        // THIS IS WRONG
        const x_dist = Math.abs(this.x - from.x)
        const y_dist = Math.abs(this.y - from.y)
        return x_dist + y_dist
    }
    move(instr: Instruction = new Instruction('U', 0) ){
        switch (instr.direction) {
            case 'R':
                return new Point( this.x + instr.distance, this.y )
            case 'U':
                return new Point( this.x, this.y + instr.distance )
            case 'L':
                return new Point( this.x - instr.distance, this.y )
            case 'D':
                return new Point( this.x, this.y - instr.distance )
            default:
                throw "Bad instruction!";
        }
    }
}