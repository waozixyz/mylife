proc update_yearly_view {config year} {
    set name [dict get $config name]
    set yearly_events [dict get $config yearly_events]
    
    wm title . "My Life - $name - Year $year"
    
    # Clear existing content
    foreach widget [winfo children .frame] {
        destroy $widget
    }
    
    # Days in each month (accounting for leap years)
    set days_in_month {31 28 31 30 31 30 31 31 30 31 30 31}
    if {[clock format [clock scan "$year-12-31" -format "%Y-%m-%d"] -format "%j"] == 366} {
        lset days_in_month 1 29
    }
    
    # Sort events by start date
    if {[dict exists $yearly_events $year]} {
        set year_events [lsort -command compare_events [dict get $yearly_events $year]]
    } else {
        set year_events {}
    }
    
    set num_events [llength $year_events]
    set today [clock seconds]
    
    # Create a grid of 12 months, each month in its own row
    for {set month 1} {$month <= 12} {incr month} {
        set days [lindex $days_in_month [expr {$month - 1}]]
        
        for {set day 1} {$day <= $days} {incr day} {
            set current_date [clock scan "$year-$month-$day" -format "%Y-%m-%d"]
            set color "white"
            
            # Find the current event
            for {set i 0} {$i < $num_events} {incr i} {
                set event [lindex $year_events $i]
                set event_start [dict get $event start]
                
                if {$i == ($num_events - 1)} {
                    set event_end $today
                } else {
                    set next_event [lindex $year_events [expr {$i + 1}]]
                    set event_end [dict get $next_event start]
                }
                
                if {$current_date >= $event_start && $current_date < $event_end} {
                    set color [dict get $event color]
                    break
                }
            }
            
            label .frame.cell$month-$day -width 2 -height 1 -bg $color -relief solid
            grid .frame.cell$month-$day -row [expr {$month - 1}] -column [expr {$day - 1}] -padx 0 -pady 0
        }
    }
    
    # Create a legend
    set row 12
    if {$year_events ne {}} {
        for {set i 0} {$i < $num_events} {incr i} {
            set event [lindex $year_events $i]
            set name [dict get $event name]
            set color [dict get $event color]
            set start [clock format [dict get $event start] -format "%Y-%m-%d"]
            
            if {$i == ($num_events - 1)} {
                set end_text "ongoing"
            } else {
                set next_event [lindex $year_events [expr {$i + 1}]]
                set end [clock format [dict get $next_event start] -format "%Y-%m-%d"]
                set end_text "to $end"
            }
            
            label .frame.legend$row -text "$name (from $start $end_text)" -bg $color -fg white -relief solid
            grid .frame.legend$row -row $row -column 0 -columnspan 31 -sticky "ew"
            incr row
        }
    }
}

proc compare_events {a b} {
    set date_a [dict get $a start]
    set date_b [dict get $b start]
    return [expr {$date_a - $date_b}]
}
