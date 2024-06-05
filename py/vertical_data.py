import nnfs
import matplotlib.pyplot as plt
import numpy as np
from nnfs.datasets import vertical_data

nnfs.init()
X, y = vertical_data(samples=100, classes=3)

# so we can easily import it in Rust
np.savetxt('out.txt', X)
np.savetxt('out_Y.txt', y)

# we will use the binary for creating a graph
# np.save("outfile_x", X)
# np.save("outfile_y", y)
