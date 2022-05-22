#[derive(Debug)]
struct Position(f64, f64, f64);

#[derive(Debug)]
struct Velocity(f64, f64, f64);

#[derive(Debug)]
struct Particle {
    mass: f64,
    position: Position,
    velocity: Velocity
}

struct Force {}

impl Force {
    fn fx(_step: f64) -> f64 {
        0.0
    }

    fn fy(_step: f64) -> f64 {
        0.0
    }

    fn fz(_step: f64) -> f64 {
        -9.8
    }
}

struct Integrator {
    step_size: f64,
}

impl Integrator {
    fn stopping_condition(step: f64, particle: &Particle) -> bool{
        step != 0.0 && (particle.position.2 <= 0.0 || step > 100000.0)
    }

    fn take_step(&self, step: f64 , particle: &mut Particle) {
        particle.velocity = Velocity(
            particle.velocity.0 + (Force::fx(step)/particle.mass)*self.step_size,
            particle.velocity.1 + (Force::fy(step)/particle.mass)*self.step_size,
            particle.velocity.2 + (Force::fz(step)/particle.mass)*self.step_size
        );
        particle.position = Position(
            particle.position.0 + particle.velocity.0*self.step_size,
            particle.position.1 + particle.velocity.1*self.step_size,
            particle.position.2 + particle.velocity.2*self.step_size
        );
    }

    fn integrate(self, particle: &mut Particle) {
        let mut step = 0.0;
        while !Integrator::stopping_condition(step, particle) {
            self.take_step(step, particle);
            step = step + self.step_size;
        }
    }
}

fn main() {
    let integrator = Integrator { step_size: 0.01 };

    let mut particle = Particle {
        mass: 1.0,
        position: Position(0.0, 0.0, 0.0),
        velocity: Velocity(20.0, 20.0, 20.0)
    };

    integrator.integrate(&mut particle);

    println!("{:?}", particle);
}