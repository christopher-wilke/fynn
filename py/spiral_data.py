import nnfs
import matplotlib.pyplot as plt
import numpy as np
from nnfs.datasets import spiral_data

nnfs.init()
X, y = spiral_data(samples=10, classes=5)

# so we can easily import it in Rust
np.savetxt('out.txt', X)
# np.savetxt('out_Y.txt', y)

# we will use the binary for creating a graph
np.save("outfile_x", X)
np.save("outfile_y", y)
