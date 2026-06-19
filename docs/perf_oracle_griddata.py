import time, numpy as np
from scipy.interpolate import griddata
side=24
pts=[]; vals=[]
for iy in range(side):
    for ix in range(side):
        x=(ix+0.3*(iy%3))/side; y=(iy+0.2*(ix%5))/side
        pts.append([x,y]); vals.append(np.sin(x*5)+np.cos(y*3))
pts=np.array(pts); vals=np.array(vals)
q=np.array([[((i*37)%997)/997.0, ((i*53+17)%991)/991.0] for i in range(1024)])
def med(fn,r=7):
    ts=[]
    for _ in range(r):
        t0=time.perf_counter(); fn(); ts.append(time.perf_counter()-t0)
    return sorted(ts)[len(ts)//2]
print("scipy griddata linear 576pts/1024q: %.2f us"%(med(lambda: griddata(pts,vals,q,method='linear'))*1e6))
