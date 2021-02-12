# Ezel

A Python plotting library written in Rust using [plotters](https://github.com/38/plotters).

- fast
- object-oriented API
- no problem with >= 100_000 points

- [Guide]()
- [API Reference]()
- [Benchmarks]

## Install

You need
- rust
- libfontconfig
- libfreetype

```
pip install ezel
```

or

```
git clone https://github.com/elbaro/ezel.git
cd ezel
pip install .
```

## Usage
```
import ezel as ez
import numpy as np

x = np.array([1.0, 3.0, 5.0])
y = np.array([7.0, 2.0, 3.0])

canvas = ez.Canvas()
left, right = canvas.split_horizontally()
c = ez.Chart(left, caption='Title Chart1', margin=10)
c.line(x, y)
canvas.save('ezel.png')
```

## vs Matplotlib

|            | n=100*100            | n=1000*1000                                                 |
|------------|----------------------|-------------------------------------------------------------|
| ezel       | 0.030641794204711914 | 1.2414026260375977                                          |
| matplotlib | 0.4168715476989746   | crash even with mpl.rcParams['agg.path.chunksize'] = n * 10 |

This is not a fair comparison but gives you a sense of how they handle large dataset.

<img src="https://github.com/elbaro/ezel/raw/main/screenshots/ezel.png" class="galleryItem" width=300px />
<img src="https://github.com/elbaro/ezel/raw/main/screenshots/matplotlib.png" class="galleryItem" width=300px />


```
import time

import numpy as np
from matplotlib import pyplot as plt

import ezel as ez

n = 1000*1000
x = np.random.randn(n)
y = np.random.randn(n)


def draw_ezel():
    canvas = ez.Canvas()
    c = ez.Chart(canvas,
                 x_range=ez.Range.f64(-10,10),
                 y_range=ez.Range.f64(-10,10),
                 caption='Title Chart1',
                 margin=10)
    c.line(x, y)
    canvas.save('ezel.png')


def draw_matplotlib():
    # without manually increasing the upper limit, matplotlib crashes

    import matplotlib as mpl
    mpl.rcParams['agg.path.chunksize'] = n * 10

    plt.plot(x, y)
    plt.savefig('matplotlib.png')


t = time.time()
draw_ezel()
print(time.time() - t)

t = time.time()
draw_matplotlib()
print(time.time() - t)
```

## Roadmap for 0.1.x
Currently only numeric scalars are supported.

- [ ] Draw x, y axis and grid by default
- [x] Accept a Python list as an argument
- [x] Accept i32, i64, f32 as arguments (by converting to f64)
- [x] Add .scatter()
- [ ] Support datetime, date, time
- [ ] line, scatter style customization
- [x] auto color rotation
- [ ] Title font customization
- [ ] Histogram
- [ ] Static Build

## Roadmap for 0.2.0
- [ ] Support other backends such as SVG and wasm
- [ ] Add `xy=` which accepts a sequence of xy pairs.
- [ ] Log-scale on x, y axis
- [ ] Draw i32, i64, f32 data without converting to f64
- [ ] Support Pandas/PyPolars DataFrames/Series
