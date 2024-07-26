package require yaml

proc read_config {filename} {
    set fh [open $filename r]
    set data [read $fh]
    close $fh
    set config [::yaml::yaml2dict $data]

    # Set default values if missing
    if {![dict exists $config name]} {
        dict set config name "Unknown"
    }
    if {![dict exists $config date_of_birth]} {
        dict set config date_of_birth "2000-01"
    }
    if {![dict exists $config life_expectancy]} {
        dict set config life_expectancy 80
    }
    if {![dict exists $config life_periods]} {
        dict set config life_periods {}
    }
    if {![dict exists $config yearly_events]} {
        dict set config yearly_events {}
    }
    return $config
}

proc get_yaml_files {} {
    set files [glob -nocomplain -directory "data" *.yaml *.yml]
    set basenames {}
    foreach file $files {
        lappend basenames [file tail $file]
    }
    return $basenames
}