import numpy as np

# Passed-in gradient from the next layer
# For demo purpose, we are going to use a vec of 1.s
dvalues = np.array([
                   [1., 1., 1.],
                   [2., 2., 2.],
                   [3., 3., 3.]
])

weights = np.array([
    [0.2, 0.8, -0.5, 1],
    [0.5, -0.91, 0.26, -0.5],
    [-0.26, -0.27, 0.17, 0.87]
]).T

dinputs = np.dot(dvalues, weights.T)
print(dinputs)
