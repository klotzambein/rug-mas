import matplotlib.pyplot as plt
import numpy as np
from scipy.stats import norm, pearsonr
import pandas as pd


data = pd.read_csv("data/combined.csv")["price[0]"]
rets = (data.shift(1) / data).dropna()
log_rets = rets.apply(np.log)
norm_log_rets = (log_rets - log_rets.mean()) / log_rets.std()

count = len(norm_log_rets)

x = np.linspace(1, 0, count)

samples = np.abs(np.array(norm_log_rets))
samples.sort()

samples_n = np.abs(norm.rvs(size=count))
samples_n.sort()

plt.scatter(samples, x, s=1.2)
plt.scatter(samples_n, x, s=1.2)
plt.yscale('log')
plt.xscale('log')
plt.xlim([0.1,10])
plt.ylabel("P>(|Ret|)")
plt.xlabel("|Ret|")
plt.grid()
plt.show()

abs_rets = np.abs((data.shift(1) - data).dropna())
print(rets, abs_rets)

tau = np.linspace(0, 100, 101, dtype=int)
ac_rets = [pearsonr(np.array(rets.shift(t)[t:]), np.array(rets[t:]))[0] for t in tau]
ac_abs_rets = [pearsonr(np.array(abs_rets.shift(t)[t:]), np.array(abs_rets[t:]))[0] for t in tau]

plt.scatter(tau, ac_rets, s=1.2, color="blue")
plt.scatter(tau, ac_abs_rets, s=1.2, color="red")
plt.grid()
plt.show()