import ezel as ez
from datetime import datetime as dt, date as d, time as t, timedelta as td

# data
x_d = [d(2020, 1, 1), d(2020, 4, 6), d(2020, 11, 12)]
x_t = [t(23, 11, 20, 555), t(11, 23, 52, 912), t(1, 0, 2, 11)]
x_dt = [dt.combine(date, time) for (date, time) in zip(x_d, x_t)]
x_td = [td(hours=1), td(hours=2, minutes=5), td(hours=4, microseconds=99)]

y = [100, 200, 150]

# draw
canvas = ez.Canvas()
top, bottom = canvas.split_vertically()
left, right = bottom.split_horizontally()
del bottom

# datetime
c = ez.Chart(top, caption='datetime vs f64', x_range=ez.Range.datetime(dt(2020, 1, 1), dt(2020, 12, 31)), y_range=ez.Range.f64(0, 200))
c.line(x_dt, y)

# date
c = ez.Chart(left, caption='date vs f64', x_range=ez.Range.date(d(2020, 1, 1), d(2020, 12, 31)), y_range=ez.Range.f64(0, 200))
c.line(x_d, y)

# duration
c = ez.Chart(right, caption='duration vs f64', x_range=ez.Range.duration(td(hours=0), td(hours=10)), y_range=ez.Range.f64(0, 200))
c.line(x_td, y)

canvas.save('example_chrono.png')
