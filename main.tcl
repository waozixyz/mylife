package require Tk
source "config_reader.tcl"
source "lifetime_view.tcl"
source "yearly_view.tcl"
source "ui_components.tcl"

# Main window
frame .frame
pack .frame -fill both -expand 1

# Parse command line arguments and set up initial config
set yaml_file ""
set config {}
set years 80
set current_year [clock format [clock seconds] -format "%Y"]

foreach arg $argv {
    if {[string match *.yaml $arg] || [string match *.yml $arg]} {
        set yaml_file $arg
    } else {
        set yaml_file "$arg.yaml"
    }
}

if {$yaml_file ne ""} {
    if {[file exists "data/$yaml_file"]} {
        set config [read_config "data/$yaml_file"]
        set years [dict get $config life_expectancy]
    } else {
        puts "Error: File 'data/$yaml_file' not found."
        exit 1
    }
}

# Set up UI components
setup_ui_components

# Initialize view
.view_frame.selector set "Lifetime"
# Only update view if config is not empty
if {$config ne {}} {
    update_view
}

# Start the event loop
tkwait window .
