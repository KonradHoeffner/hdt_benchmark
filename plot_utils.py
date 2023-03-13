import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import sys

import warnings
warnings.filterwarnings('ignore', message='.*converting a masked element to nan.*')

alias = {
    "librdf": "librdf (c)",
    "jena": "jena (java)",
    "n3js": "n3js (js)",
    "python": "rdflib (python)",
    "pypy": "rdflib (pypy)",
}

color_key = {
    "sophia": "red",
    "sophia_lg": "darkorange",
    "sophia_hdt": "yellow",
    "hdt_rs": "fuchsia",
    "hdt_java": "darkgray",
    "hdt_cpp": "lightgreen",
    "librdf (c)": "purple",
    "jena (java)": "black",
    "n3js (js)": "blue",
    "sophia_wasm": "darkorange",
    "sophia_wasm_lg": "red",
    "rdflib (python)": "green",
    "rdflib (pypy)": "darkgreen",
}

def load_data(task, *tools):
    dfs = []
    for tool in tools:
        try:
            df = pd.read_csv("csv/{}-{}.csv".format(task, tool))
            df['tool'] = alias.get(tool, tool)
            dfs.append(df)
        except FileNotFoundError as ex:
            print(ex, file=sys.stderr)
    df = pd.concat(dfs)
    df.index = range(len(df.index))
    
    if task[:5] == 'query':
        df['t_query'] = (df.t_first + df.t_rest)
        df['r_load'] = (df['size'] / df.t_load)
    elif task == 'parse':
        df['r_parse'] = (df['size'] / df.t_parse)
    return df.groupby(['tool', 'size'])

def my_plot(data, attr_name, *, exclude=[], savename=None, color_key=color_key, fig=None, **kw):
    means = data[attr_name].mean().unstack().transpose()
    stdev = data[attr_name].std().unstack().transpose()
    for i in exclude:
        try:
            del means[i]
            del stdev[i]
        except:
            pass
    color = list(means.columns.map(color_key.get))
    ax = means.plot(yerr=2*stdev, grid=1, color=color, **kw)
    if savename:
        ax.get_figure().savefig("figures/{}.svg".format(savename))
    return ax

def plot_query_stats(data, color_key=color_key, group=False, task="query"):
    figw = FIGW
    figh = FIGH
    if group:
        _, (ax0, ax1) = plt.subplots(figsize=(figw*2, figh), nrows=1, ncols=2)
    else:
        (ax0, ax1) = (None, None)

    if task=="query":
        my_plot(data, "t_load", title="Time (in s) to load an NT/HDT file in memory", loglog=True, color_key=color_key, ax=ax0)
        #my_plot(data, "t_load", xlim=(0,200_000), ylim=(0,10), savename="t_load_lin", title="Time (in s) to load an NT file in memory", ax=ax0)
        my_plot(data, "r_load", title="Load rate (in triple/s) from an NT/HDT file in memory", logx=True, color_key=color_key, ax=ax1)

        if group:
            _, (ax0, ax1) = plt.subplots(figsize=(figw*2, figh), nrows=1, ncols=2)
        else:
            (ax0, ax1) = (None, None)

        my_plot(data, 'm_graph', title="Memory (in kB, RSS) used while allocating for the graph", loglog=True, color_key=color_key, ax=ax0)
        my_plot(data, 't_query', xlim=(9_000_000,10_350_000), ylim=(0.26,0.38), title="Time (in s) to retrieve all matching triples (*,p,o), excerpt" , loglog=False, color_key=color_key, ax=ax1)
    
        if group:
            _, (ax0, ax1) = plt.subplots(figsize=(figw*2, figh), nrows=1, ncols=2)
        else:
            (ax0, ax1) = (None, None)

    if task=="query":
        pattern = "(*,p,o)"
    else:
        pattern = "(s,*,*)" 

    my_plot(data, 't_first', title="Time (in s) to retrieve the first matching triple " + pattern, loglog=True, color_key=color_key, ax=ax0)
    my_plot(data, 't_query', title="Time (in s) to retrieve all matching triples " + pattern, loglog=True, color_key=color_key, ax=ax1)
    
    #my_plot(data, 't_query', xlim=(0,1_000_000), ylim=(0, 0.1), title="Time (in s) to retrieve all matching triples (*,p,o)", savename="t_query_lin", ax=ax1)

def plot_table(*tools):
    dfs = []
    for tool in tools:
        try:
            df = pd.read_csv("csv/{}-{}.csv".format("query", tool))
            df = df[df['size'] == 10310000]
            df = df.mean(numeric_only=True).to_frame().T
            df['tool'] = alias.get(tool, tool)

            dfs.append(df)
        except FileNotFoundError as ex:
            print(ex, file=sys.stderr)
    df = pd.concat(dfs)
    df.index = range(len(df.index))
    
    df['t_query'] = (df.t_first + df.t_rest)
    df['r_load'] = (df['size'] / df.t_load)
    df['m_graph'] = (df['m_graph'] / 1000).round()
    df = df.filter(items=['tool', 'm_graph', 't_load', 't_query'])

    fig, ax = plt.subplots()
    # hide axes
    fig.patch.set_visible(False)
    ax.axis('off')
    ax.axis('tight')
    #df = pd.DataFrame(np.random.randn(10, 4), columns=list('ABCD'))
    table = ax.table(cellText=df.values, colLabels=df.columns, loc='center')
    table.scale(5, 5)
    #table.auto_set_font_size(False)
    #table.set_fontsize(14)
    #fig.tight_layout()
    plt.show()
   
def plot_parse_stats(data, color_key=color_key, group=False):
    figw = FIGW
    figh = FIGH
    if group:
        _, (ax0, ax1) = plt.subplots(figsize=(figw*2, figh), nrows=1, ncols=2)
    else:
        (ax0, ax1) = (None, None)
    my_plot(data, "t_parse", loglog=True, title="Time (in s) to parse an NT file", color_key=color_key, ax=ax0)
    my_plot(data, "r_parse", title="Parse rate (in triple/s) from an NT file in memory", logx=True, color_key=color_key, ax=ax1)

FIGW=7
FIGH=4
