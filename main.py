import tkinter as tk
from ui_components import setup_ui_components
from config_reader import read_config
from views import update_view
import sys
import os
import signal
import ttkbootstrap as ttk
from ttkbootstrap.constants import *


def signal_handler(sig, frame):
    print("\nExiting application...")
    root.quit()
    sys.exit(0)

# Register the signal handler
signal.signal(signal.SIGINT, signal_handler)

def shutdown_program(event=None):
    print("\nExiting application...")
    root.quit()
    sys.exit(0)


# Main window
root = ttk.Window(themename="superhero")  

root.bind('<Control-q>', shutdown_program)
root.bind('<Mod4-q>', shutdown_program)

# Global variables
config = {}
years = 80

# Parse command line arguments and set up initial config
yaml_file = ""
for arg in sys.argv[1:]:
    if arg.endswith(('.yaml', '.yml')):
        yaml_file = arg
    else:
        yaml_file = f"{arg}.yaml"

if yaml_file:
    if os.path.exists(f"data/{yaml_file}"):
        config = read_config(f"data/{yaml_file}")
        years = config['life_expectancy']
    else:
        print(f"Error: File 'data/{yaml_file}' not found.")
        sys.exit(1)

# Set up UI components
top_bar, yaml_selector, view_selector, year_selector = setup_ui_components(root, config, years)


# Create main frame after top bar
frame = tk.Frame(root)
frame.pack(fill=tk.BOTH, expand=1)

# Initialize view
if config:
    update_view(root, frame, config, years, view_selector, year_selector)

# Function to handle window close
def on_closing():
    root.quit()
    sys.exit(0)

# Bind the closing event
root.protocol("WM_DELETE_WINDOW", on_closing)

# Start the event loop
try:
    root.mainloop()
except KeyboardInterrupt:
    print("\nExiting application...")
    root.quit()
    sys.exit(0)