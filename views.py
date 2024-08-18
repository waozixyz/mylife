import tkinter as tk
from tkinter import ttk
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

    # Create a grid of specified years, 12 months per year, 4 years per row
    for i in range(rows):
        for j in range(48):
            current_date = dob + timedelta(days=(i*48 + j)*30)  # Approximation
            color = "white"

            # Check which life period this month belongs to
            for period_index, period in enumerate(life_periods):
                start = datetime.strptime(period['start'], "%Y-%m")
                end = datetime.now() if period_index == period_count - 1 else datetime.strptime(life_periods[period_index + 1]['start'], "%Y-%m")
                if start <= current_date < end:
                    color = period['color']
                    break

            label = ttk.Label(frame, width=1, background=color, relief='solid')
            label.grid(row=i, column=j, padx=0, pady=0)

    # Create a legend
    row = rows
    for period in life_periods:
        label = ttk.Label(frame, text=f"{period['name']} (from {period['start']})", background=period['color'], foreground='white', relief='solid')
        label.grid(row=row, column=0, columnspan=48, sticky="ew")
        row += 1
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
    
    # Create a grid of 13 rows, each with 28 days
    for row in range(13):
        for col in range(28):
            day_of_year = row * 28 + col + 1
            if day_of_year <= days_in_year:
                current_date = year_start + timedelta(days=day_of_year - 1)
                color = "white"
                
                # Find the current event
                for i, event in enumerate(year_events):
                    event_start = event['start']
                    event_end = date(year + 1, 1, 1) if i == num_events - 1 else year_events[i+1]['start']
                    
                    if event_start <= current_date < event_end:
                        color = event['color']
                        break
                
                label = ttk.Label(frame, width=2, background=color, relief='solid')
                label.grid(row=row, column=col, padx=0, pady=0)
            else:
                # Empty cell for days beyond the year
                label = ttk.Label(frame, width=2, background="gray", relief='solid')
                label.grid(row=row, column=col, padx=0, pady=0)
    
    # Create a legend
    legend_row = 13
    for i, event in enumerate(year_events):
        name = event.get('location', 'Unknown')
        color = event['color']
        start = event['start'].strftime("%Y-%m-%d")
        
        end_text = "ongoing" if i == num_events - 1 else f"to {year_events[i+1]['start'].strftime('%Y-%m-%d')}"
        
        label = ttk.Label(frame, text=f"{name} ({start} {end_text})", background=color, foreground='white', relief='solid')
        label.grid(row=legend_row, column=0, columnspan=28, sticky="ew")
        legend_row += 1
    
    logger.debug(f"Yearly view updated with {num_events} events")