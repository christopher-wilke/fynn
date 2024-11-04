import numpy as np

# Passed-in gradient from the next layer
# for the purpose of this example we're going to use
# an array of an incremental gradient values
dvalues = np.array([[1., 1., 1.],
                    [2., 2., 2.],
                    [3., 3., 3.]
])

# We have 3 sets of inputs - samples
inputs = np.array([
                  [1, 2, 3, 2.5],
                  [2., 5., -1., 2],
                  [-1.5, 2.7, 3.3, -0.8]
])

# We have 3 sets of weights - one set for each neuron
# we have 4 inputs, thus 4 weights
# recall that we keep weights transposed
weights = np.array([[0.2, 0.8, -0.5, 1],
    [0.5, -0.91, 0.26,-0.5],
    [-0.26, -0.27, 0.17, 0.87]
]).T

biases = np.array([[2, 3, 0.5]])

# Forward Pass
layer_outputs = np.dot(inputs, weights) + biases # Dense Layer
relu_outputs = np.maximum(0, layer_outputs) # ReLu Activation

# Let's optimize and test backpropagation here
drelu = relu_outputs.copy()
drelu[layer_outputs <= 0] = 0 # Doesnt makse sense to me as we already did ReLU

# Dense Layer 
# dinputs - multiply by weights
dinputs = np.dot(drelu, weights.T)
# dweights - multiply by inputs
dweights = np.dot(inputs.T, drelu)
# dbiases - sum values, keepdims
dbiases = np.sum(drelu, axis=0, keepdims=True)

# update params
weights += -0.001*dweights
biases += -0.001*dbiases

print(weights)
print(biases)
