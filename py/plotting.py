import matplotlib.pyplot as plt
import numpy as np

def f(x):
    return 2*x**2

#np.arrange(start, stop, step) to give us smoother line
x = np.arange(0, 5, 0.001)
y = f(x)

plt.plot(x, y)

colors = ['k', 'g', 'r', 'b', 'c']

def approximate_tangent_line(x, approximate_derivative):
    return (approximate_derivative*x)+b

for i in range(5):

    p2_delta = 0.0001
    x1 = i
    x2 = x1+p2_delta

    y1 = f(x1)
    y2 = f(x2)

    print((x1, y1), (x2, y2))

    # Derivative approximation and y-intercept for the tangent line
    approximate_derivative = (y2-y1)/(x2-x1)
    b = y2-approximate_derivative*x2

    # Putting the tangent line calcultation into a function so we can call it multiple times
    def tangent_line(x):
        return approximate_derivative*x + b

    # Plotting the tangent line
    # +/- 0.9 to draw the tangent line on our graph
    # then we calculate y for given x using the tangent line function
    to_plot = [x1-0.9, x1, x1+0.9]
    plt.scatter(x1, y1, c=colors[i])

    plt.plot([point for point in to_plot], 
             [approximate_tangent_line(point, approximate_derivative) for point in to_plot],
             c=colors[i])


    print(f'Approximate derivative for f(x) where x = {x1} is {approximate_derivative}')

plt.show()
