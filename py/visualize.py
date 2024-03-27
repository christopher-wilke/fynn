import matplotlib.pyplot as plt
import numpy as np

file_path = "example.txt"

X = np.load("outfile.npy")

plt.scatter(X[:, 0], X[:, 1])
plt.show()
