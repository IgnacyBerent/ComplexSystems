import random
import math
import numpy as np
from animate import animate_simulation


class Boid:
    def __init__(self, x, y, vx, vy, field_of_view=2 * np.pi):
        self.position = np.array([x, y], dtype=float)
        self.velocity = np.array([vx, vy], dtype=float)
        self.trajectories = np.array([self.position])
        self.update_velocity = np.array([0.0, 0.0])
        self.field_of_view = field_of_view

    def move(self):
        self.velocity += self.update_velocity
        self.position += self.velocity
        self.trajectories = np.append(self.trajectories, [self.position], axis=0)

    def avoid_collision(self, boids, avoid_radius, max_avoid_force=1):
        # Avoid collisions with nearby boids
        avoidance_vector = np.array([0.0, 0.0])
        for other_boid in boids:
            if other_boid is self:
                continue
            distance = np.linalg.norm(self.position - other_boid.position)
            if distance < avoid_radius and distance > 0:
                direction_to_other = other_boid.position - self.position
                angle = math.acos(
                    np.dot(self.velocity, direction_to_other)
                    / (
                        np.linalg.norm(self.velocity)
                        * np.linalg.norm(direction_to_other)
                    )
                )
                if angle < self.field_of_view / 2:
                    avoidance_vector += (self.position - other_boid.position) / distance

        if np.linalg.norm(avoidance_vector) > 0:
            avoidance_vector = (
                avoidance_vector / np.linalg.norm(avoidance_vector) * max_avoid_force
            )

        return avoidance_vector

    def velocity_matching(self, boids, align_radius, max_align_force):
        # Match velocity with nearby boids
        avg_velocity = np.array([0.0, 0.0])
        boids_in_radius = 0
        for other_boid in boids:
            if other_boid is self:
                continue
            distance = np.linalg.norm(self.position - other_boid.position)
            if distance < align_radius and distance > 0:
                direction_to_other = other_boid.position - self.position
                angle = math.acos(
                    np.dot(self.velocity, direction_to_other)
                    / (
                        np.linalg.norm(self.velocity)
                        * np.linalg.norm(direction_to_other)
                    )
                )
                if angle < self.field_of_view / 2:
                    avg_velocity += other_boid.velocity
                    boids_in_radius += 1

        if boids_in_radius > 0:
            avg_velocity /= boids_in_radius
            alignment_vector = avg_velocity - self.velocity
            alignment_force = (
                alignment_vector / np.linalg.norm(alignment_vector) * max_align_force
            )

            return alignment_force

        return np.array([0.0, 0.0])

    def flock_centering(self, boids, centering_radius, max_centering_force):
        # Move towards the center of the flock

        center_of_mass = np.array([0.0, 0.0])
        birds_in_radius = 0

        for other_boid in boids:
            if other_boid is self:
                continue
            distance = np.linalg.norm(self.position - other_boid.position)
            if distance < centering_radius and distance > 0:
                direction_to_other = other_boid.position - self.position
                angle = math.acos(
                    np.dot(self.velocity, direction_to_other)
                    / (
                        np.linalg.norm(self.velocity)
                        * np.linalg.norm(direction_to_other)
                    )
                )
                if angle < self.field_of_view / 2:
                    center_of_mass += other_boid.position
                    birds_in_radius += 1

        if birds_in_radius > 0:
            center_of_mass /= birds_in_radius
            centering_vector = center_of_mass - self.position
            centering_force = (
                centering_vector
                / np.linalg.norm(centering_vector)
                * max_centering_force
            )
            return centering_force

        return np.array([0.0, 0.0])

    def avoid_borders(self, border_limit):
        border_avoidance_vector = np.array([0.0, 0.0])
        if (
            self.position[0] + self.velocity[0] > border_limit
            or self.position[0] + self.velocity[0] < -border_limit
        ):
            border_avoidance_vector[0] = -self.velocity[0] * 2
        if (
            self.position[1] + self.velocity[1] > border_limit
            or self.position[1] + self.velocity[1] < -border_limit
        ):
            border_avoidance_vector[1] = -self.velocity[1] * 2

        self.update_velocity += border_avoidance_vector

    def calculate_new_velocity(
        self,
        boids,
        max_avoid_force,
        max_align_force,
        max_centering_force,
        avoid_radius,
        align_radius,
        centering_radius,
        avoid_weight,
        align_weight,
        centering_weight,
        border_limit,
    ):
        avoidance_force = self.avoid_collision(boids, avoid_radius, max_avoid_force)
        align_force = self.velocity_matching(boids, align_radius, max_align_force)
        centering_force = self.flock_centering(
            boids, centering_radius, max_centering_force
        )
        self.update_velocity = (
            avoidance_force * avoid_weight
            + align_force * align_weight
            + centering_force * centering_weight
        )
        self.avoid_borders(border_limit)


def init_birds(n, spawn_radius, velocity_range, field_of_view):
    birds = []
    for i in range(n):
        x = random.randint(-spawn_radius, spawn_radius)
        y = random.randint(-spawn_radius, spawn_radius)
        vx = random.random() * random.choice([-1, 1]) * velocity_range
        vy = random.random() * random.choice([-1, 1]) * velocity_range
        birds.append(Boid(x, y, vx, vy, field_of_view))
    return birds


def main(ic: dict):
    birds = init_birds(
        ic["n_of_birds"],
        ic["spawn_radius"],
        ic["velocity_range"],
        ic["field_of_view"],
    )
    for i in range(ic["time"]):
        for bird in birds:
            bird.move()

        for bird in birds:
            bird.calculate_new_velocity(
                birds,
                max_avoid_force=ic["max_avoid_force"],
                max_align_force=ic["max_align_force"],
                max_centering_force=ic["max_centering_force"],
                avoid_radius=ic["avoid_radius"],
                align_radius=ic["align_radius"],
                centering_radius=ic["centering_radius"],
                avoid_weight=ic["avoid_weight"],
                align_weight=ic["align_weight"],
                centering_weight=ic["centering_weight"],
                border_limit=ic["animation_size"],
            )

    trajectories = [bird.trajectories for bird in birds]
    animate_simulation(
        trajectories=trajectories,
        size=ic["animation_size"],
        t=np.arange(0, ic["time"]),
        title=ic["filename"],
    )


if __name__ == "__main__":
    ic = {
        "filename": "flocking_simulation_360view",
        "time": 500,
        "n_of_birds": 40,
        "animation_size": 1000,
        "spawn_radius": 500,
        "velocity_range": 15,
        "field_of_view": 2 * np.pi,
        "max_avoid_force": 1,
        "avoid_radius": 10,
        "avoid_weight": 0.3,
        "max_align_force": 2,
        "align_radius": 75,
        "align_weight": 0.5,
        "max_centering_force": 1,
        "centering_radius": 100,
        "centering_weight": 0.3,
    }
    main(ic)
