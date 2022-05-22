#[derive(Debug)]
struct Position(f64, f64, f64);

#[derive(Debug)]
struct Velocity(f64, f64, f64);

#[derive(Debug)]
struct Particle {
    mass: f64,
    time: f64,
    position: Position,
    velocity: Velocity
}

struct Force {}

impl Force {
    fn fx(_time: f64) -> f64 {
        0.0
    }

    fn fy(_time: f64) -> f64 {
        0.0
    }

    fn fz(_time: f64) -> f64 {
        -9.8
    }
}

struct IntegrationStats {
    total_steps: i32
}

struct Integrator {
    step_duration: f64,
}

impl Integrator {
    fn stopping_condition(step: i32, particle: &Particle) -> bool{
        step != 0 && (particle.position.2 <= 0.0 || step > 100000)
    }

    fn take_step(&self, particle: &mut Particle) {
        particle.velocity = Velocity(
            particle.velocity.0 + (Force::fx(particle.time)/particle.mass)*self.step_duration,
            particle.velocity.1 + (Force::fy(particle.time)/particle.mass)*self.step_duration,
            particle.velocity.2 + (Force::fz(particle.time)/particle.mass)*self.step_duration
        );
        particle.position = Position(
            particle.position.0 + particle.velocity.0*self.step_duration,
            particle.position.1 + particle.velocity.1*self.step_duration,
            particle.position.2 + particle.velocity.2*self.step_duration
        );
        particle.time += self.step_duration;
    }

    fn integrate(self, particle: &mut Particle) -> IntegrationStats {
        let mut step = 0;
        while !Integrator::stopping_condition(step, particle) {
            self.take_step(particle);
            step += 1;
        }

        IntegrationStats {
            total_steps: step
        }
    }
}

fn main() {
    let integrator = Integrator { step_duration: 0.01 };

    let mut particle = Particle {
        mass: 1.0,
        time: 0.0,
        position: Position(0.0, 0.0, 0.0),
        velocity: Velocity(20.0, 20.0, 20.0)
    };

    let stats = integrator.integrate(&mut particle);

    println!("Integration took {} steps.", stats.total_steps);
    println!("Final state: {:#?}", particle);
}