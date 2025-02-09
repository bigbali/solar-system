pub fn collosion_system(collisions: EventReader<Collision>) {
    not_implemented!()

    // body makes available:
    // energy: mass & velocity
    // force dir: velocity1 - velocity2

    // emit event from axisting n-body system to avoid comparing twice

    // if bodies collide, release energy
    // ? negative mass maybe?
    // determine released energy
    // if energy large enough, destroy one or both bodies
    // body applies force to other body, pushing it away
}

struct Collision {
    pub body_a: (&Body, Entity),
    pub body_b: (&Body, Entity),
}
