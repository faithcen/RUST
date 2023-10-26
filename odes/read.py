import pandas as pd
import matplotlib.pyplot as plt

# Read data
data = pd.read_csv('output.txt')

# Plot data
plt.figure(figsize=(10, 6))
for col in ['y1', 'y2', 'y3', 'y4', 'y5', 'y6']:
    plt.plot(data['time'], data[col], label=col)

plt.xlabel('time')
plt.ylabel('y values')
plt.legend()
plt.title('Kepler Orbit Data')
plt.grid(True)
plt.show()
