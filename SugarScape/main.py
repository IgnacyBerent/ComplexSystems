import numpy as np
import matplotlib.pyplot as plt
from matplotlib.ticker import MaxNLocator
from matplotlib.colors import LinearSegmentedColormap
import random

yellow_alpha_cmap = LinearSegmentedColormap.from_list(
    "yellow_alpha_cmap", [(1, 1, 0, alpha) for alpha in np.linspace(0, 1, 256)]
)


class SugarScapeAgent:
    """
    A class representing an agent in the SugarScape model.

    Attributes:
      pos (numpy.ndarray): The position of the agent on the grid.
      metabolism (int): Ammount of sugar consumed by the agent per step.
      vision (int): The distance the agent can see.
      speed (int): The number of steps the agent can take per iteration.
    """

    def __init__(self, pos):
        self.pos = pos
        self.sugar = random.randint(5, 25)
        self.metabolism = random.randint(1, 4)
        self.vision = random.randint(1, 6)


class SugarCell:
    """
    A class representing a cell in the SugarScape model.

    Attributes:
      sugar (int): The ammount of sugar in the cell.
      capacity (int): The maximum ammount of sugar the cell can store.
      growth_rate (int): The ammount of sugar the cell grows per step.
    """

    def __init__(self, capacity, growth_rate):
        self.capacity = capacity
        self.growth_rate = growth_rate
        self.sugar = self._init_sugar()

    def _init_sugar(self):
        """
        Initialize the sugar in the cell to be at maximum capacity.
        """
        return self.capacity

    def grow(self):
        """
        Grow the sugar in the cell.
        """
        self.sugar = min(self.sugar + self.growth_rate, self.capacity)


class SugarScapeSimulaiton:
    """
    A class representing a simulation of the SugarScape model.

    Attributes:
      size (int): The size of the grid.
      agents (list): A list of agents in the simulation.
      sugar_cells (np.array(SugarCell)): A 2D array of sugar cells in the simulation.
    """

    def __init__(self, size, num_agents):
        self.size = size
        self.agents = self._init_agents(num_agents)
        self.sugar_cells = None

    def _init_agents(self, num_agents):
        """
        Initialize agents in random unique positions on the grid.
        """
        all_posible_positions = [
            (x, y) for x in range(self.size) for y in range(self.size)
        ]
        positions = random.sample(all_posible_positions, num_agents)
        agents = [SugarScapeAgent(pos) for pos in positions]
        return agents

    def _move_agent(self, agent):
        """
        Move the agent to the unoccupied cell with the most sugar in its vision, if draw then move to the closest cell, if draw then choose randomly.
        """
        x, y = agent.pos
        vision = agent.vision
        max_sugar = self.sugar_cells[y, x].sugar
        max_sugar_pos_list = [(x, y)]
        for iy in range(max(y - vision, 0), min(y + vision + 1, self.size)):
            for ix in range(max(x - vision, 0), min(x + vision + 1, self.size)):
                if self.sugar_cells[iy, ix].sugar > max_sugar and any(
                    [(ix, iy) != a.pos for a in self.agents]
                ):
                    max_sugar = self.sugar_cells[iy, ix].sugar
                    max_sugar_pos_list = [(ix, iy)]
                elif self.sugar_cells[iy, ix].sugar == max_sugar and any(
                    [(ix, iy) != a.pos for a in self.agents]
                ):
                    max_sugar_pos_list.append((ix, iy))
        if len(max_sugar_pos_list) == 1:
            agent.pos = max_sugar_pos_list[0]
        elif len(max_sugar_pos_list) > 1:
            distances = [
                np.linalg.norm(np.array(pos) - np.array(agent.pos))
                for pos in max_sugar_pos_list
            ]
            min_distance = min(distances)
            min_distance_count = distances.count(min_distance)
            if min_distance_count == 1:
                agent.pos = max_sugar_pos_list[distances.index(min_distance)]
            else:
                equal_distance_positions = [
                    max_sugar_pos_list[i]
                    for i in range(len(max_sugar_pos_list))
                    if distances[i] == min_distance
                ]
                agent.pos = random.choice(equal_distance_positions)

    def _consume_sugar(self, agent):
        """
        Consume sugar from the cell the agent is in, do metabolism and remove the agent if it starves.
        """
        x, y = agent.pos
        agent.sugar += self.sugar_cells[y, x].sugar - agent.metabolism
        self.sugar_cells[y, x].sugar = 0
        if agent.sugar < 0:
            self.agents.remove(agent)

    def step(self):
        """
        Perform a step in the simulation.
        """
        for agent in self.agents:
            # Move the agent
            self._move_agent(agent)
            # Consume sugar
            self._consume_sugar(agent)

        # Grow sugar
        for row in self.sugar_cells:
            for cell in row:
                cell.grow()

    def plot(self):
        """
        Plot the simulation.
        """
        fig, ax = plt.subplots()
        cax = ax.imshow(
            [[cell.sugar for cell in row] for row in self.sugar_cells],
            cmap=yellow_alpha_cmap,
        )
        colorbar = fig.colorbar(cax, ax=ax, orientation="vertical", label="Sugar Level")
        colorbar.locator = MaxNLocator(integer=True)
        colorbar.update_ticks()
        ax.invert_yaxis()
        for agent in self.agents:
            ax.scatter(*agent.pos, color="red", s=1)
        plt.show()

    def init_sugar_cells_random(self, capacity_range, growth_rate_range):
        """
        Initialize sugar cells with random sugar and capacity.
        """
        self.sugar_cells = np.array(
            [
                [
                    SugarCell(
                        random.randint(*capacity_range),
                        random.randint(*growth_rate_range),
                    )
                    for _ in range(self.size)
                ]
                for _ in range(self.size)
            ]
        )


if __name__ == "__main__":
    random.seed(33)
    sim = SugarScapeSimulaiton(100, 400)
    sim.init_sugar_cells_random((1, 4), (1, 1))
    sim.plot()
    for _ in range(10):
        sim.step()
        sim.plot()
