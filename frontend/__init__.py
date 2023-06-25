import os

import dash
import dash_core_components as dcc
import dash_html_components as html
from dash.dependencies import Input, Output, State
import plotly.graph_objs as go

# Initialize the Dash application
app = dash.Dash(__name__)

# Define the layout of the application
app.layout = html.Div([
    html.H1("Distributed Collatz Sequence Visualizer Database"),

    # Upload section
    dcc.Upload(
        id='upload-data',
        children=html.Div([
            'Drag and Drop or ',
            html.A('Select Files')
        ]),
        style={
            'width': '50%',
            'height': '60px',
            'lineHeight': '60px',
            'borderWidth': '1px',
            'borderStyle': 'dashed',
            'borderRadius': '5px',
            'textAlign': 'center',
            'margin': '10px'
        },
        multiple=False
    ),

    # Output section
    html.Div(id='output-container', style={'margin': '20px'}),

    # Visualization section
    dcc.Graph(id='collatz-visualization')
])


def verify(image_id: str, receipt_file: str) -> None:
    """
    image_id: str
        The ID of the image to verify.
        "[2784318997,2318768834,960732258,1118058729,98960111,548153307,3286462278,3557308076]"

    receipt_file: str
        The path to the receipt file, i.e. "receipt.dat".
        Paths can be either relative or absolute.

    Raises an exception if the receipt was not verified.
    """
    out = os.system(f'cargo run -- "{image_id}" "{receipt_file}"')
    if out != 0:
        raise Exception('Receipt not verified.')



# Callback to handle the uploaded data
@app.callback(Output('output-container', 'children'),
              [Input('upload-data', 'contents')],
              [State('upload-data', 'filename')])
def process_upload(contents, filename):
    if contents is not None:
        # Perform verification and sequence processing here

        # Store the sequence and verification result in the database
        return html.Div([
            html.H4(f'Uploaded file: {filename}'),
            html.P('Verification passed. Sequence stored in the database.')
        ])
    else:
        return html.Div()


# Callback to generate the Collatz sequence visualization
@app.callback(Output('collatz-visualization', 'figure'),
              [Input('output-container', 'children')])
def generate_visualization(upload_output):
    # Retrieve the stored sequence from the database

    if not verified:
        # error
        return

    # Generate a visualization using Plotly or any other library

    # Placeholder example visualization
    y_values = [2, 4, 8, 16, 32]
    x_values = list(range(1, len(y_values)))

    trace = go.Scatter(
        x=x_values,
        y=y_values,
        mode='lines+markers',
        name='Collatz Sequence'
    )

    layout = go.Layout(
        title='Collatz Sequence Visualization',
        xaxis=dict(title='Iteration'),
        yaxis=dict(title='Value')
    )

    return {'data': [trace], 'layout': layout}


# Run the application
if __name__ == '__main__':
    app.run_server(debug=True)
