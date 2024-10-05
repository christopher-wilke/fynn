# import numpy as np
import numpy as np

# Input arrays
inputs = np.array([
    [0.17640523612499237, 0.5978738069534302],
    [0.040015723556280136, 0.7240893244743347]
])
weights = np.array([
    [0.002473880972289464, 0.014895691422809047, -0.0044469715755695045],
    [0.008707085360891076, -0.0064183642832156855, -0.0018901445751785932]
])

# Perform matrix multiplication
result = np.dot(inputs, weights)
print(result)
