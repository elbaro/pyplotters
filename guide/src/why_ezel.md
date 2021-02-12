# Why Ezel?

Ezel best suits the everyday data science plotting, for example you have pandas DataFrame with 10'000~10'000'000 rows and you want to quickly visualize them in a few seconds.

Goals
- fast
- simple api for everyday data science
- provide useful primitives for analysis such as error bar, violin plot, etc.
- reasonable look-and-feel and customization

Non-goals
- fancy interactivity (more than zoom or 3d navigation)
- beautiful plot

## Comparisons

- matplotlib: matplotlib has performance traps. While it is lightening-fast to draw a million points with `plt.plot(range(1000000), range(1000000)); plt.savefig(..)` and I assumed matplotlib has no problem handling a million points, sometimes it struggles to draw only 100'000 points in a few minutes. Sometimes simply adding a legend takes several minutes because matplotlib does some calculation to find the best legend location. There were many cases I still don't know the exact reason for slowness. Even if I can solve the issue, it is hard to remember the solution and there are so many traps. (I still lookup `mpl.rcParams['agg.path.chunksize']`)
- seaborn, plotnine and many others: these use matplotlib as their backend, therefore slower than matplotlib.
- vispy and other gl libs: They cover realtime animation and 3Ds but require you to write some gl code. vispy's high-level api is in progres.
- altair: I really liked the grammar but its rendering relies on the web browser. It is slower than matplotlib. Vega+Svg cannot beat the bitmap's performance because the browser renders 1'000'000 dom elements everytime you open the plot. Rendering to bitmap files requires a headless browser engine.
