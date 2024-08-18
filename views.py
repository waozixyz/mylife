import tkinter as tk
from ttkbootstrap import Style, Label
from ttkbootstrap.constants import *
import logging
from datetime import datetime, timedelta, date

logger = logging.getLogger(__name__)

def update_view(root, frame, config, years, view_selector, year_selector):
    view = view_selector.get()
    logger.debug(f"Updating view: {view}")
    if view == "Lifetime":
        update_timeline(root, frame, config, years)
    elif view == "Yearly":
        year = int(year_selector.get())
        logger.debug(f"Updating yearly view for year: {year}")
        update_yearly_view(root, frame, config, year)
def create_legend(frame, items):
    legend_frame = tk.Frame(frame)
    legend_frame.pack(side=tk.BOTTOM, fill=tk.X, padx=10, pady=10)
    for item in items:
        color = item['color']
        text = item['text']
        item_frame = tk.Frame(legend_frame, bg=color)
        item_frame.pack(side=tk.TOP, fill=tk.X, pady=2)
        label = Label(item_frame, text=text, background=color, foreground='black', anchor='w')
        label.pack(side=tk.TOP, fill=tk.X, padx=5, pady=2)
        
def update_timeline(root, frame, config, years):
    name = config.get('name', 'Unknown')
    dob = datetime.strptime(config['date_of_birth'], "%Y-%m")
    life_periods = config['life_periods']
    period_count = len(life_periods)

    root.title(f"My Life - {name}")

    # Clear existing grid
    for widget in frame.winfo_children():
        widget.destroy()

    # Calculate rows based on years parameter
    rows = (years + 3) // 4

    # Create a canvas
    canvas = tk.Canvas(frame)
    canvas.pack(side=tk.TOP, fill=tk.BOTH, expand=True)

    def draw_grid(event=None):
        canvas.delete("all")
        width = canvas.winfo_width()
        height = canvas.winfo_height()
        cell_size = min(width // 48, height // rows)

        for i in range(rows):
            for j in range(48):
                current_date = dob + timedelta(days=(i*48 + j)*30)  # Approximation
                color = "white"

                for period_index, period in enumerate(life_periods):
                    start = datetime.strptime(period['start'], "%Y-%m")
                    end = datetime.now() if period_index == period_count - 1 else datetime.strptime(life_periods[period_index + 1]['start'], "%Y-%m")
                    if start <= current_date < end:
                        color = period['color']
                        break
                canvas.create_rectangle(j*cell_size, i*cell_size, (j+1)*cell_size, (i+1)*cell_size, fill=color, outline='black')

    canvas.bind("<Configure>", draw_grid)

    # Create a legend
    legend_items = [{'color': period['color'], 'text': f"{period['name']} (from {period['start']})"} for period in life_periods]
    create_legend(frame, legend_items)

def update_yearly_view(root, frame, config, year):
    name = config.get('name', 'Unknown')
    yearly_events = config.get('yearly_events', {})
    
    logger.debug(f"Updating yearly view for {name}, year {year}")
    logger.debug(f"All yearly events: {yearly_events}")
    
    year_events = yearly_events.get(year, [])
    
    logger.debug(f"Events for year {year}: {year_events}")
    
    root.title(f"My Life - {name} - Year {year}")
    
    # Clear existing content
    for widget in frame.winfo_children():
        widget.destroy()
    
    # Days in the year (accounting for leap years)
    days_in_year = 366 if year % 4 == 0 and (year % 100 != 0 or year % 400 == 0) else 365
    
    # Sort events by start date
    year_events = sorted(year_events, key=lambda x: x['start'])
    
    num_events = len(year_events)
    year_start = date(year, 1, 1)
    
    # Create a canvas
    canvas = tk.Canvas(frame)
    canvas.pack(side=tk.TOP, fill=tk.BOTH, expand=True)

    def draw_grid(event=None):
        canvas.delete("all")
        width = canvas.winfo_width()
        height = canvas.winfo_height()
        cell_size = min(width // 28, height // 13)

        for row in range(13):
            for col in range(28):
                day_of_year = row * 28 + col + 1
                if day_of_year <= days_in_year:
                    current_date = year_start + timedelta(days=day_of_year - 1)
                    color = "white"
                    
                    for i, event in enumerate(year_events):
                        event_start = event['start']
                        event_end = date(year + 1, 1, 1) if i == num_events - 1 else year_events[i+1]['start']
                        
                        if event_start <= current_date < event_end:
                            color = event['color']
                            break
                    
                    canvas.create_rectangle(col*cell_size, row*cell_size, (col+1)*cell_size, (row+1)*cell_size, fill=color, outline='black')
                else:
                    canvas.create_rectangle(col*cell_size, row*cell_size, (col+1)*cell_size, (row+1)*cell_size, fill='gray', outline='black')

    canvas.bind("<Configure>", draw_grid)
    
    # Create a legend
    legend_items = [{'color': event['color'], 'text': f"{event['name']} (from {event['start'].strftime('%Y-%m-%d')})"} for event in year_events]
    create_legend(frame, legend_items)
    
    logger.debug(f"Yearly view updated with {num_events} events")