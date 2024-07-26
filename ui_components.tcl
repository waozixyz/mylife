proc setup_ui_components {} {
    global config years current_year

    # Create selector for YAML files
    set yaml_files [get_yaml_files]
    ttk::combobox .selector -values $yaml_files -state readonly
    pack .selector -pady 10
    bind .selector <<ComboboxSelected>> {
        set selected_file [.selector get]
        set config [read_config "data/$selected_file"]
        set years [dict get $config life_expectancy]
        update_view
    }

    # View selector
    frame .view_frame
    pack .view_frame -pady 10

    label .view_frame.label -text "View:"
    ttk::combobox .view_frame.selector -values {"Lifetime" "Yearly"} -state readonly
    pack .view_frame.label -side left
    pack .view_frame.selector -side left -padx 5

    # Year selector (for yearly view)
    frame .year_frame
    pack .year_frame -pady 10

    label .year_frame.label -text "Year:"
    ttk::combobox .year_frame.selector -values {} -state readonly
    pack .year_frame.label -side left
    pack .year_frame.selector -side left -padx 5

    # Initially hide the year selector content
    .year_frame.label configure -state disabled
    .year_frame.selector configure -state disabled

    # Bind events
    bind .view_frame.selector <<ComboboxSelected>> {
        set view [.view_frame.selector get]
        if {$view eq "Yearly"} {
            set yearly_events [dict get $config yearly_events]
            set year_list [lsort -integer [dict keys $yearly_events]]
            .year_frame.selector configure -values $year_list
            if {[llength $year_list] > 0} {
                .year_frame.selector set [lindex $year_list 0]
            }
            .year_frame.label configure -state normal
            .year_frame.selector configure -state readonly
        } else {
            .year_frame.label configure -state disabled
            .year_frame.selector configure -state disabled
        }
        update_view
    }

    bind .year_frame.selector <<ComboboxSelected>> {
        update_view
    }
}

proc update_view {} {
    global config years
    set view [.view_frame.selector get]
    if {$view eq "Lifetime"} {
        update_timeline $config $years
    } elseif {$view eq "Yearly"} {
        set selected_year [.year_frame.selector get]
        if {$selected_year ne ""} {
            update_yearly_view $config $selected_year
        }
    }
}
