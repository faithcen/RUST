import pandas as pd
import matplotlib.pyplot as plt

# Read data
data = pd.read_csv('output.txt')

# Plot data
plt.figure(figsize=(10, 6))
for col in ['y1', 'y2', 'y3']:
    plt.plot(data['time'], data[col], label=col)

plt.xlabel('Time')
plt.ylabel('Concentration')
plt.title('Concentration vs Time')
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.show()
