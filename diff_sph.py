import sys, scipy.special as sp
worst={}
for line in sys.stdin:
    p=line.strip().split(',')
    fn=p[0]
    try:
        if fn in ('pro_cv','obl_cv'):
            m,n,c=int(p[1]),int(p[2]),float(p[3]); v=float(p[4])
            ref=float(sp.pro_cv(m,n,c) if fn=='pro_cv' else sp.obl_cv(m,n,c))
            key=fn; loc=f'm{m}n{n}c{c}'
        else:
            m,n,c,x=int(p[1]),int(p[2]),float(p[3]),float(p[4]); v=float(p[5]); vp=float(p[6])
            f=getattr(sp,fn); rc,rcp=f(m,n,c,x); rc=float(rc)
            e=abs(v-rc)/max(abs(rc),1e-300) if abs(rc)>1e-8 else 0.0
            key=fn; loc=f'm{m}n{n}c{c}x{x}'
            if e>worst.get(key,(0,))[0]: worst[key]=(e,loc,v,rc)
            continue
    except Exception as ex:
        continue
    e=abs(v-ref)/max(abs(ref),1e-300)
    if e>worst.get(key,(0,))[0]: worst[key]=(e,loc,v,ref)
for fn in sorted(worst):
    e,loc,v,ref=worst[fn]
    print(f'{fn}: worst_rel={e:.2e} @ {loc} got={v:.6e} ref={ref:.6e}'+(' <-- BAD' if e>1e-6 else ''))
