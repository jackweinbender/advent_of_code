import { Wire } from "./wire"
import { Vector } from "./vector";
import { Point } from "./point"

export class Panel {
    static intersections(left: Wire, right: Wire){
        let intersections: Set<Point> = new Set();

        left.path.forEach(l_vector => {
            right.path.forEach( r_vector => {
                const intersection = Vector.intersection(l_vector, r_vector)
                if(intersection){ intersections.add(intersection) }
            });
        });
        return intersections
    }
}