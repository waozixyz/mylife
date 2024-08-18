import tkinter as tk

from ttkbootstrap import Style, Label
from ttkbootstrap.constants import *

from tkinter import ttk
from datetime import datetime
from config_reader import get_yaml_files, read_config
from views import update_view
import logging

# Set up logging
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)
def setup_ui_components(root, config, years):
    # Create a top bar
    top_bar = ttk.Frame(root)
    top_bar.pack(fill=tk.X, pady=10)

    yaml_files = get_yaml_files()
    yaml_selector = ttk.Combobox(top_bar, values=yaml_files, width=30, bootstyle="default")
    yaml_selector.set("Select YAML file")
    yaml_selector.pack(side=tk.LEFT, padx=10)

    view_selector = ttk.Combobox(top_bar, values=["Lifetime", "Yearly"], width=15, bootstyle="default")
    view_selector.set("Lifetime")
    view_selector.pack(side=tk.LEFT, padx=10)

    year_selector = ttk.Combobox(top_bar, width=10, bootstyle="default")
    update_year_selector(year_selector, config)
    year_selector.pack(side=tk.LEFT, padx=10)
    year_selector.pack_forget()  # Hide initially

    # Now bind the events after all widgets are created
    yaml_selector.bind("<<ComboboxSelected>>", lambda event: on_yaml_change(event, root, config, years, yaml_selector, view_selector, year_selector))
    view_selector.bind("<<ComboboxSelected>>", lambda event: on_view_change(event, root, config, years, yaml_selector, view_selector, year_selector))
    year_selector.bind("<<ComboboxSelected>>", lambda event: on_year_change(event, root, config, years, yaml_selector, view_selector, year_selector))

    return top_bar, yaml_selector, view_selector, year_selector

def on_yaml_change(event, root, config, years, yaml_selector, view_selector, year_selector):
    yaml_file = yaml_selector.get()
    if yaml_file and yaml_file != "Select YAML file":
        config.clear()
        config.update(read_config(f"data/{yaml_file}"))
        years = config.get('life_expectancy', 80)
        update_year_selector(year_selector, config)
        logger.debug(f"YAML file changed to: {yaml_file}")
        logger.debug(f"Config: {config}")
        frame = root.winfo_children()[-1]  # Get the last child (main frame)
        update_view(root, frame, config, years, view_selector, year_selector)

def on_view_change(event, root, config, years, yaml_selector, view_selector, year_selector):
    view = view_selector.get()
    if view == "Lifetime":
        year_selector.pack_forget()
    elif view == "Yearly":
        year_selector.pack(side=tk.LEFT)
    logger.debug(f"View changed to: {view}")
    frame = root.winfo_children()[-1]  # Get the last child (main frame)
    update_view(root, frame, config, years, view_selector, year_selector)

def on_year_change(event, root, config, years, yaml_selector, view_selector, year_selector):
    year = int(year_selector.get())
    logger.debug(f"Year changed to: {year}")
    frame = root.winfo_children()[-1]  # Get the last child (main frame)
    update_view(root, frame, config, years, view_selector, year_selector)

def update_year_selector(year_selector, config):
    yearly_events = config.get('yearly_events', {})
    available_years = sorted(yearly_events.keys(), reverse=True)
    year_selector['values'] = available_years
    if available_years:
        year_selector.set(available_years[0])
    logger.debug(f"Available years: {available_years}")

    