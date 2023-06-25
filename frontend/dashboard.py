from collections import defaultdict
from typing import List

import dash
import dash_core_components as dcc
import dash_html_components as html
import plotly.express as px
import plotly.graph_objs as go
import requests
from dash.dependencies import Input, Output
from typing_extensions import TypedDict

# Initialize the Dash application
app = dash.Dash(__name__)

# Define the layout of the application
app.layout = html.Div([
    html.H1("Collatz Sequence Visualizer"),

    html.Div([
        html.Div(
            children=[
                html.Label("Enter a natural number:"),
                dcc.Input(
                    id="number-input",
                    type="number",
                    placeholder="Enter a number",
                    value=None,
                ),
                dcc.Graph(id='collatz-visualization'),
            ],
            style={"width": "50%", "display": "inline-block"},
        ),

        html.Div(
            children=[
                html.Label("Convergence Length Range:"),
                dcc.Input(
                    id="min-length-input",
                    type="number",
                    placeholder="Min Length",
                    value=1,
                ),
                dcc.Input(
                    id="max-length-input",
                    type="number",
                    placeholder="Max Length",
                    value=1_000,
                ),
                dcc.Graph(id="convergence-distribution"),
            ],
            style={"width": "50%", "display": "inline-block"},
            # style={"margin-top": "20px"},
        ),

    ], style={
        'display': 'flex',
    }),
])


class Row(TypedDict):
    output_sequence: List[int]
    proof: bool


# Callback to generate the Collatz sequence visualization
@app.callback(
    Output('collatz-visualization', 'figure'),
    [
        Input("number-input", "value")
    ],
)
def generate_visualization(number):
    if number is None:
        return {}

    # Fetch the sequence from the database
    result: Row = requests.get(f'http://localhost:8000/public/data/{number}').json()
    if 'output_sequence' not in result:
        return {}
    sequence = result['output_sequence']

    # Placeholder example visualization
    x_values = list(range(1, len(sequence)))

    trace = go.Scatter(
        x=x_values,
        y=sequence,
        mode='lines+markers',
        name='Collatz Sequence'
    )

    layout = go.Layout(
        title='Collatz Sequence Visualization',
        xaxis=dict(title='Iteration'),
        yaxis=dict(title='Value')
    )

    return {'data': [trace], 'layout': layout}


@app.callback(
    Output("convergence-distribution", "figure"),
    [
        Input("min-length-input", "value"),
        Input("max-length-input", "value")
    ],
)
def plot_distribution(min_length, max_length):
    if min_length is not None and max_length is not None:

        _result = requests.get(f'http://localhost:8000/public/data?min={min_length}&max={max_length}').json()
        result = _result['data']
        print(f'Number of sequences to plot: {len(result)}')

        # Calculate the frequency of stopping times using the Collatz sequences:
        stopping_times = defaultdict(int)
        for sequence in result.values():
            stopping_times[len(sequence)] += 1

        return px.histogram(
            x=list(stopping_times.keys()),
            labels={"x": "Stopping Time", "y": "Frequency"},
            title="Stopping Time Frequency",
        )

    # Return an empty figure if the button is not clicked or if the input values are not provided
    return {}


def populate_database():
    def collatz_sequence(n: int) -> List[int]:
        """
        Returns the Collatz sequence starting from n.
        """
        sequence = [n]
        while n != 1:
            if n % 2 == 0:
                n //= 2
            else:
                n = 3 * n + 1
            sequence.append(n)
        return sequence

    # Populate the database with the first 1000 Collatz sequences
    for i in range(1, 1_000):
        seq = collatz_sequence(i)
        # print(i, len(seq))
        requests.post(
            url=f'http://localhost:8000/public/data/actions/create',
            json=dict(
                output_sequence=seq,
                proof=[],
                image_id=[]
            ),
        ),


# Run the application
if __name__ == '__main__':
    populate_database()
    app.run_server(debug=True)
