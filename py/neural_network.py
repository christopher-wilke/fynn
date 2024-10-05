import numpy as np
import nnfs
from nnfs.datasets import spiral_data, vertical_data

nnfs.init()

# Dense Layer
class Layer_Dense:

    # Layer Init
    def __init__(self, n_inputs, n_neurons, demo=False):
        if not demo:
            self.weights = 0.01*np.random.randn(n_inputs, n_neurons)
            self.biases = np.zeros((1, n_neurons))
        else:
            print('now using demo data')
            d_w = np.array(
                [
                    [0.01764052,  0.00400157,  0.00978738],
                    [0.02240893,  0.01867558, -0.00977278]
                ]
            )
            self.weights = d_w
            self.biases = np.zeros((1, n_neurons))

    # Forward Pass
    def forward(self, inputs):
        print(f'inputs={inputs}')
        print(f'self.weights={self.weights}')
        print(f'np.dot={np.dot(inputs, self.weights)}')
        self.output = np.dot(inputs, self.weights) + self.biases

# ReLu Activation
class Activation_ReLU:

    # Forward Pass
    def forward(self, inputs):
        self.output = np.maximum(0, inputs)

# Softmax Activation
class Activation_Softmax:

    # Forward Pass
    def forward(self, inputs):
        exp_values = np.exp(inputs - np.max(inputs, axis=1, keepdims=True))
        probabilities = exp_values / np.sum(exp_values, axis=1, keepdims=True)
        self.output = probabilities

# Common Loss Class
class Loss:

    # Calculates the data and regularization losses
    # given model and ground truth values
    def calculate(self, output, y):

        # Calculate sample losses
        sample_losses = self.forward(output, y)

        # Calculate mean loss
        data_loss = np.mean(sample_losses)

        return data_loss

class Loss_Categorical_Crossentropy(Loss):

    def forward(self, y_pred, y_true):

        # Number of samples in a batch
        samples = len(y_pred)

        # Clip data to prevent division by 0
        y_pred_clipped = np.clip(y_pred, 1e-7, 1 - 1e-7)

        if len(y_true.shape) == 1:
            correct_confidences = y_pred_clipped[
                range(samples),
                y_true
            ]
        elif len(y_true.shape) == 2:
            correct_confidences = np.sum(
                y_pred_clipped*y_true, axis=1
            )

        # Losses
        negative_log_likelihoods = -np.log(correct_confidences)
        return negative_log_likelihoods

X, y = vertical_data(samples=2, classes=1)

np.savetxt('out.txt', X)
np.savetxt('out_Y.txt', y)

dense1 = Layer_Dense(2, 3)
dense1.forward(X)

# print(f'X={X[0]}')
# print(f'weight={dense1.weights}')
# print(f'dense1.output={dense1.output[0]}')

# activation1 = Activation_ReLU()
# activation1.forward(dense1.output)

# dense2 = Layer_Dense(3, 3)
# activation2 = Activation_Softmax()

# # Create Loss Function
# loss_function = Loss_Categorical_Crossentropy()
# dense2.forward(activation1.output)
# activation2.forward(dense2.output)

# loss =loss_function.calculate(activation2.output, y)

# # calculate accuracy
# predictions = np.argmax(activation2.output, axis=1)

# # Helpers
# lowest_loss = 99999999
# best_dense1_weights = dense1.weights.copy()
# best_dense1_biases = dense1.biases.copy()
# best_dense2_weights = dense2.weights.copy()
# best_dense2_biases = dense2.biases.copy()

# for iteration in range(1000):

#     # Update weights with some small random values
#     dense1.weights += 0.5*np.random.randn(2, 3)
#     dense1.biases += np.random.rand(1, 3)
#     dense2.weights += 0.5*np.random.randn(3, 3)
#     dense2.biases += 0.5*np.random.randn(1, 3)

#     # Perform a forward pass of our training data
#     dense1.forward(X)
#     activation1.forward(dense1.output)
#     dense2.forward(activation1.output)
#     activation2.forward(dense2.output)
    
#     # return loss
#     loss =loss_function.calculate(activation2.output, y)

#     # calculate accuracy
#     predictions = np.argmax(activation2.output, axis=1)
#     accuracy = np.mean(predictions == y)

#     # If loss is smaller, print and save weights
#     if loss < lowest_loss:
#         print('New set of weights found, iteration: ', iteration, 'loss: ', loss, 'acc: ', accuracy)
#         best_dense1_weights = dense1.weights.copy()
#         best_dense1_biases = dense1.biases.copy()
#         best_dense2_weights = dense2.weights.copy()
#         best_dense2_biases = dense2.biases.copy()
#         lowest_loss = loss
#     else:
#         dense1.weights = best_dense1_weights.copy()
#         dense1.biases = best_dense1_biases.copy()
#         dense2.weights = best_dense2_weights.copy()
#         dense2.biases = best_dense2_biases.copy()
