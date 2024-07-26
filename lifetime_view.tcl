proc update_timeline {config years} {
    set name [dict get $config name]
    set dob [dict get $config date_of_birth]
    set dob_seconds [clock scan "${dob}-01" -format "%Y-%m-%d"]
    set life_periods [dict get $config life_periods]
    set period_count [llength $life_periods]

    wm title . "My Life - $name"

    # Clear existing grid
    foreach widget [winfo children .frame] {
        destroy $widget
    }

    # Calculate rows based on years parameter
    set rows [expr {($years + 3) / 4}]

    # Create a grid of specified years, 12 months per year, 4 years per row
    for {set i 0} {$i < $rows} {incr i} {
        for {set j 0} {$j < 48} {incr j} {
            set current_date [clock add $dob_seconds [expr {$i * 48 + $j}] months]
            set year [clock format $current_date -format "%Y"]
            set month [clock format $current_date -format "%m"]
            set color "white"

            # Check which life period this month belongs to
            for {set period_index 0} {$period_index < $period_count} {incr period_index} {
                set period [lindex $life_periods $period_index]
                set start [clock scan "[dict get $period start]-01" -format "%Y-%m-%d"]
                set end [expr {$period_index == ($period_count - 1) ?
                               [clock seconds] :
                               [clock scan "[dict get [lindex $life_periods [expr {$period_index + 1}]] start]-01" -format "%Y-%m-%d"]}]
                if {$current_date >= $start && $current_date < $end} {
                    set color [dict get $period color]
                    break
                }
            }

            label .frame.cell$i-$j -width 1 -height 1 -bg $color -relief solid
            grid .frame.cell$i-$j -row $i -column $j -padx 0 -pady 0
        }
    }

    # Create a legend
    set row $rows
    foreach period $life_periods {
        set name [dict get $period name]
        set color [dict get $period color]
        set start [dict get $period start]

        label .frame.legend$row -text "$name (from $start)" -bg $color -fg white -relief solid
        grid .frame.legend$row -row $row -column 0 -columnspan 48 -sticky "ew"
        incr row
    }
}