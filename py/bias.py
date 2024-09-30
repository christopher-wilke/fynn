import numpy as np

inputs = [
    [1., 2.],
    [3., 4.],
    [5., 6.],
    [7., 8.]
]
weights = [
    [0.2, 0.8, -0.5, 1],
    [0.5, -0.91, 0.26, -0.5],
    [-0.26, -0.27, 0.17, 0.87],
    [0.3, 0.1, -0.41, 0.8]]
# biases = [2, 3, 0.5, 0.2]

# layer_outputs = []

# for neuron_weights, neuron_bias in zip(weights, biases):
#     neuron_output = 0.
#     for n_input, weight in zip(inputs, neuron_weights):
#         neuron_output += n_input*weight
#     # neuron_output += neuron_bias
#     layer_outputs.append(neuron_output)

# layer_outputs = np.dot(weights, inputs)
# print(layer_outputs)
print(np.dot(weights, inputs))  
