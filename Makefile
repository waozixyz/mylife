# Variables
APP_NAME = life
WRAPPER_DIR = $(HOME)/bin
WRAPPER_PATH = $(WRAPPER_DIR)/$(APP_NAME)
NIX_SHELL = nix-shell
SHELL_NIX = shell.nix

.PHONY: all install clean

all: install

install: $(WRAPPER_PATH)

$(WRAPPER_PATH): $(SHELL_NIX) life.tcl config.yaml
	mkdir -p $(WRAPPER_DIR)
	echo "#!/bin/sh" > $(WRAPPER_PATH)
	echo "$(NIX_SHELL) $(SHELL_NIX) --run 'tclsh life.tcl'" >> $(WRAPPER_PATH)
	chmod +x $(WRAPPER_PATH)
	@echo "Wrapper script installed at $(WRAPPER_PATH)"

clean:
	rm -f $(WRAPPER_PATH)
	@echo "Wrapper script removed from $(WRAPPER_PATH)"
