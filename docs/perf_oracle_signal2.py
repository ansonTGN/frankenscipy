import time, numpy as np
from scipy.signal import fftconvolve, remez, welch
def sig(n): return np.array([np.sin(37*i/n)+0.35*np.cos(91*i/n)+0.1*((i*17%29)-14) for i in range(n)])
def med(fn,r=9):
    ts=[]
    for _ in range(r):
        t0=time.perf_counter(); fn(); ts.append(time.perf_counter()-t0)
    return sorted(ts)[len(ts)//2]
x=sig(4096); h=sig(257)
print("scipy fftconvolve 4096x257 same: %.1f us"%(med(lambda: fftconvolve(x,h,'same'))*1e6))
try:
    print("scipy remez 257 2band: %.1f us"%(med(lambda: remez(257,[0,0.2,0.3,0.5],[1,0],[1,10],fs=2))*1e6))
except Exception as e: print("remez err:",e)
print("scipy welch 4096 w256 o128: %.1f us"%(med(lambda: welch(x,1.0,'hann',256,128))*1e6))
