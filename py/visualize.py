import matplotlib.pyplot as plt
import numpy as np

file_path = "example.txt"

X = np.load("outfile_x.npy")
y = np.load("outfile_y.npy")

plt.scatter(X[:, 0], X[:, 1], c=y, cmap='brg')
plt.show()
