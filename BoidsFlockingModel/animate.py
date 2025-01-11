import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import random


def animate_simulation(trajectories, size, t, title):
    n_of_particles = len(trajectories)

    # Create figure and axis
    fig, ax = plt.subplots(figsize=(8, 8))
    ax.set_xlim(-size, size)
    ax.set_ylim(-size, size)
    ax.set_aspect("equal")
    ax.grid(False)

    # Initialize lines for particles and trails
    colors = ["black"] * n_of_particles
    particles = [
        ax.plot([], [], "o", color=c, markersize=5)[0] for i, c in enumerate(colors)
    ]
    trails = [ax.plot([], [], "-", color=c, alpha=0.3)[0] for c in colors]

    def init():
        """Initialize animation"""
        for particle, trail in zip(particles, trails):
            particle.set_data([], [])
            trail.set_data([], [])
        return particles + trails

    def update(frame):
        """Update animation"""
        trail_length = 50
        for i, (particle, trail) in enumerate(zip(particles, trails)):
            # Update particle position
            particle.set_data([trajectories[i][frame, 0]], [trajectories[i][frame, 1]])

            # Update trail
            start = max(0, frame - trail_length)
            trail.set_data(
                trajectories[i][start : frame + 1, 0],
                trajectories[i][start : frame + 1, 1],
            )
        return particles + trails

    # Create animation
    anim = FuncAnimation(
        fig, update, frames=len(t), init_func=init, blit=True, interval=20, repeat=True
    )

    filename = title.replace(" ", "_").lower()
    anim.save(f"{filename}.gif", writer="ffmpeg", fps=30)

    plt.show()
    return anim  # Keep a reference to prevent garbage collection
