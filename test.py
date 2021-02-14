import time

import numpy as np

import ezel as ez

n = 1000*10000
x = np.clip(np.random.randn(n), -10, 10)
y = np.clip(np.random.randn(n), -10, 10)


def draw_ezel():
    canvas = ez.Canvas()
    c = ez.Chart(canvas,
                 x_range=ez.Range.f64(-10,10),
                 y_range=ez.Range.f64(-10,10),
                 caption='Title Chart1',
                 margin=10)
    c.line(x, y, stroke_width=1)
    canvas.save('ezel.png')


def draw_matplotlib():
    import matplotlib
    # matplotlib.use('cairo')
    matplotlib.use('GTK3Agg')


    import matplotlib as mpl
    mpl.rcParams['agg.path.chunksize'] = n * 10
    from matplotlib import pyplot as plt

    plt.rcParams['agg.path.chunksize'] = n * 10
    plt.plot(x, y)
    plt.savefig('matplotlib.png')


t = time.time()
draw_ezel()
print(time.time() - t)

t = time.time()
draw_matplotlib()
print(time.time() - t)