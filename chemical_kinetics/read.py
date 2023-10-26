import numpy as np
import matplotlib.pyplot as plt

# Path to the output.txt file relative to the src folder
file_path = 'output.txt'

# Load data from output.txt
data = np.loadtxt(file_path, delimiter=',', skiprows=1)

# Extract columns: time, concentrations of X, Y, and Z
time = data[:, 0]
x = data[:, 1]
y = data[:, 2]
z = data[:, 3]

# Plot
plt.figure(figsize=(10, 6))
plt.plot(time, x, label='X', color='blue')
plt.plot(time, y, label='Y', color='red')
plt.plot(time, z, label='Z', color='green')
plt.xlabel('Time')
plt.ylabel('Concentration')
plt.title('Concentration vs Time')
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.show()
