import numpy as np

# Passed-in gradient from the next layer
# For demo purpose, we are going to use a vec of 1.s
dvalues = np.array([
                   [1., 1., 1.],
                   [2., 2., 2.],
                   [3., 3., 3.]
])

# 3 sets of input - samples
inputs = np.array([
    [1, 2, 3, 2.5],
    [2., 5., -1., 2],
    [-1.5, 2.7, 3.3, -0.8]
])

dweights = np.dot(inputs.T, dvalues)

dbiases = np.sum(dvalues, axis=0, keepdims=True)
print(dbiases)
