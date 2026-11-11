CC ?= gcc
CFLAGS ?= -O2 -Wall -Wextra

all: tabtrans gen_fixture

tabtrans: tabtrans.c
	$(CC) $(CFLAGS) -o $@ $<

gen_fixture: gen_fixture.c
	$(CC) $(CFLAGS) -o $@ $<

test: all
	./tests/run.sh

clean:
	rm -f tabtrans gen_fixture

.PHONY: all test clean
