import math
import numpy as np
import matplotlib.pylab as plt


def turn_angle(n, angle, twist):
    """Compute the turn angle based on whether n is even or odd.

    Args:

    n: number
    angle: Angle in degrees; return -angle (clockwise) if n is even.
    twist: Multiplier; return angle*twist if n is odd."""

    if n % 2 == 0:
        return -angle
    else:
        return angle * twist


def harriss_plot(ax, n, angle=10, twist=2, *args, **kwargs):
    """The Harris visualisation of the orbit of a given Collatz number, n.

    Args:

    ax:    The plotting axis (matplotlib)
    n:     The Collatz number whose orbit we wish to visualise.
    angle: The line segments for even numbers use -angle (clockwise)
    twist: The odd numbers use an angle of angle*twist (anti-clockwise).
    """

    # Reverse the orbit to visualise from teh root (1)
    orbit = collatz_orbit(n)[::-1]

    # The origin and initial heading.
    xs, ys, heading = [0], [0], 0

    # Build up the lists of x and y coords.
    for i, o in enumerate(orbit):

        # Update the current heading.
        heading += turn_angle(o, angle=angle, twist=twist)

        # Add the new (x, y)
        xs.append(xs[-1] + math.cos(math.radians(heading)))
        ys.append(ys[-1] + math.sin(math.radians(heading)))

    # Plot the coordinates as a line graph.
    ax.plot(xs, ys, *args, **kwargs)