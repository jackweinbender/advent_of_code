import { Instruction } from "./instruction"

export class Point {
    x: number
    y: number

    constructor(x: number, y: number) {
        this.x = x
        this.y = y
    }

    move(instr: Instruction = new Instruction('U', 0) ){
        switch (instr.direction) {
            case 'L':
                return new Point( this.x - instr.distance, this.y )
            case 'R':
                return new Point( this.x + instr.distance, this.y )
            case 'U':
                return new Point( this.x, this.y + instr.distance )
            case 'D':
                return new Point( this.x, this.y - instr.distance )
            default:
                throw "Bad instruction!";
        }
    }
}