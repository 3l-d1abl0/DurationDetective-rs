SHELL=/bin/bash

.DEFAULT_GOAL := list

# Color Console Output
RESET=\033[0m
GREENBOLD=\033[1;32m
YELLOWBOLD=\033[1;33m
DOT := $(shell ffmpeg --version 2>/dev/null)


list:
	@echo -e "${GREENBOLD}Targets in this Makefile:${RESET}"
	@echo ""
	@LC_ALL=C $(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]" && $$1 !~ "^[list.]" && $$1 !~ "^[always.]") {print "make "$$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$@$$'
	@echo ""
	@echo ""

duration-detective-sample-run:
	@echo -e "${GREEBOLD}Running with sample data:${RESET}"
	./target/release/duration-detective-rs --path="files/"

duration-detective-build:
	
	@echo -e "Checking if ${GREENBOLD}ffmpeg${RESET} is installed ..."
	@if command -v ffmpeg > /dev/null; then \
        echo -e "\033[1;32mffmpeg is installed.\033[0m"; \
    else \
        echo -e "\033[1;31mffmpeg is not installed. Please install it via https://www.ffmpeg.org/download.html\033[0m"; \
        exit 1; \
    fi

	@echo -e "${GREENBOLD}Linting ...${RESET}"
	cargo clippy
	@echo -e "${GREENBOLD}Running Tests ...${RESET}"
	cargo test
	@echo -e "${GREENBOLD}Building binary ...${RESET}"
	cargo build --release
	@echo -e "executable placed at \`${YELLOWBOLD}target/release/duration-detective-rs${RESET}\`"
	@echo -e "${GREENBOLD}Try running${RESET} : ${YELLOWBOLD} ./duration-detective-rs --path=\"<path-to-your-folder>\"${RESET}"

duration-detective-test:
	@echo -e "${GREENBOLD}Running Tests ...${RESET}"
	cargo test