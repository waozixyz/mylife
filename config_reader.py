import yaml
import os
from datetime import datetime, date

def read_config(filename):
    with open(filename, 'r') as file:
        config = yaml.safe_load(file)
    
    # Set default values if missing
    config.setdefault('name', 'Unknown')
    config.setdefault('date_of_birth', '2000-01')
    config.setdefault('life_expectancy', 80)
    config.setdefault('life_periods', [])
    config.setdefault('yearly_events', {})
    
    # Process yearly events
    processed_yearly_events = {}
    for year, events in config['yearly_events'].items():
        year = int(year)  # Ensure year is an integer
        processed_events = []
        for event in events:
            # Convert start date to datetime.date object if it's a string
            if isinstance(event['start'], str):
                event['start'] = datetime.strptime(event['start'], "%Y-%m-%d").date()
            elif not isinstance(event['start'], date):
                raise ValueError(f"Invalid date format for event: {event}")
            processed_events.append(event)
        processed_yearly_events[year] = processed_events
    
    config['yearly_events'] = processed_yearly_events
    
    return config

def get_yaml_files():
    files = [f for f in os.listdir('data') if f.endswith(('.yaml', '.yml'))]
    return files