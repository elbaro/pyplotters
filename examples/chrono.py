import ezel as ez
from datetime import datetime as dt, date as d, time as t

# data
x_d = [d(2020, 1, 1), d(2020, 4, 6), d(2020, 11, 12)]
x_t = [t(23, 11, 20, 555), t(11, 23, 52, 912), t(1, 0, 2, 11)]
x_dt = [dt.combine(date, time) for (date, time) in zip(x_d, x_t)]

y = [100, 200, 150]

# draw
canvas = ez.Canvas()
top, bottom = canvas.split_vertically()
left, right = bottom.split_horizontally()
del bottom

c = ez.Chart(top, x_range=ez.Range.datetime(d(2020, 1, 1), d(2020, 12, 31)), y_range=ez.Range.f64(0, 200))
c.line(x_dt, y)
c.save('example_datetime.png')

canvas = ez.Canvas()
c = ez.Chart(left, x_range=ez.Range.datetime(d(2020, 1, 1), d(2020, 12, 31)), y_range=ez.Range.f64(0, 200))
c.line(x_dt, y)
c.save('example_date.png')

canvas = ez.Canvas()
c = ez.Chart(right, x_range=ez.Range.datetime(d(2020, 1, 1), d(2020, 12, 31)), y_range=ez.Range.f64(0, 200))
c.line(x_dt, y)
c.save('example_time.png')
