# Basic Code Example

```py
import ezel as ez

# 1. Create a `Canvas` (optionally split them into multiple `Canvas`)
canvas = ez.Canvas()  
left, right = canvas.split_horizontally()

# 2. Define a `Chart` on this `Canvas` (title, x/y coordinate range, ..)
chart1 = ez.Chart(     
    left,
    x_range=ez.Range.f64(0, 10),
    y_range=ez.Range.f64(0, 10))

x = [1, 2, 3]
y = np.array([10, 15, 7])

# 3. Draw lines, scatters, or histograms on the `Chart`
#    x, y will be cast to chart1's dtype (f64)
chart1.line(x, y)     
chart1.scatter(x, y)  
chart1.hist(x, y)


# (TODO) datetime data
chart2 = ez.Chart(
    right,
    x_range=ez.Range.datetime(..),
    y_range=ez.Range.f64(-100, 100),
)
chart2.line(ez.NanoTimsetamp(x), y)
chart2.line(ez.Time(x), y)  # only use 'time' part of datetime

# 4. Save the root `Canvas`
canvas.save('plot.png')    
```
