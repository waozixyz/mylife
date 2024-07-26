# Variables
APP_NAME = life
WRAPPER_DIR = $(HOME)/bin
WRAPPER_PATH = $(WRAPPER_DIR)/$(APP_NAME)
NIX_SHELL = nix-shell
SHELL_NIX = shell.nix
CC = gcc
CFLAGS = -Wall -Wextra -pedantic
LIBS = -lraylib -lm -lpthread -ldl -lrt

.PHONY: all install clean tcl c

all: tcl c

tcl: $(WRAPPER_PATH)

c: $(APP_NAME)_c

install: tcl c

$(WRAPPER_PATH): $(SHELL_NIX) main.tcl config.yaml
	mkdir -p $(WRAPPER_DIR)
	echo "#!/bin/sh" > $(WRAPPER_PATH)
	echo "$(NIX_SHELL) $(SHELL_NIX) --run 'tclsh main.tcl'" >> $(WRAPPER_PATH)
	chmod +x $(WRAPPER_PATH)
	@echo "Tcl wrapper script installed at $(WRAPPER_PATH)"

$(APP_NAME)_c: life.c
	$(CC) $(CFLAGS) -o $@ $< $(LIBS)
	@echo "C version compiled as $(APP_NAME)_c"

clean:
	rm -f $(WRAPPER_PATH)
	rm -f $(APP_NAME)_c
	@echo "Wrapper script removed from $(WRAPPER_PATH)"
	@echo "C executable removed"

run_tcl:
	$(NIX_SHELL) $(SHELL_NIX) --run 'tclsh life.tcl'

run_c: $(APP_NAME)_c
	./$(APP_NAME)_c
	