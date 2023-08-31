###############################################################################
#
# Requires gnu make
#
###############################################################################

.PHONY: clean gcov tags

CC        = gcc
LD        = gcc
CFLAGS   += -Wall -g -O2 -fomit-frame-pointer
GCOVOPT   = -fprofile-arcs -ftest-coverage
GPROFOPT  = -pg

GEN_DATA_EXEC_NAME = gendata
GEN_DATA_SRCS      = gendata.c
GEN_DATA_OBJS      = $(patsubst %.c,%.o,$(GEN_DATA_SRCS))

PERF_EXEC_NAME     = perf
PERF_SRCS          = timsort.c \
                     timsort1.c \
                     perf.c
PERF_OBJS          = $(patsubst %.c,%.o,$(PERF_SRCS))

# Default target
all: $(PERF_EXEC_NAME) $(GEN_DATA_EXEC_NAME)

$(PERF_EXEC_NAME) : $(PERF_OBJS)
	$(LD) $(LDFLAGS) -o $@ $^

$(GEN_DATA_EXEC_NAME) : $(GEN_DATA_OBJS)
	$(LD) $(LDFLAGS) -o $@ $^

$(GEN_DATA_OBJS) : %.o : %.c 
	$(CC) $(CFLAGS) -o $@ -c $<

$(PERF_OBJS) : %.o : %.c 
	$(CC) $(CFLAGS) -o $@ -c $<

# Generating dependency files
%.d : %.c
	@$(CC) -MM $< > $@

# Include dependency file to have gcc recompile necessary sources
-include $(patsubst %.c,%.d,$(wildcard *.c))

clean:
	rm -f *.o *.d core* $(GEN_DATA_EXEC_NAME) $(PERF_EXEC_NAME) *.gcda *.gcno *.gcov

gcov:
	make clean all LDFLAGS='$(GCOVOPT)' CFLAGS='$(GCOVOPT)'

gprof:
	make clean all LDFLAGS='$(GPROFOPT)' CFLAGS='$(GPROFOPT)'

tags:
	ctags -R .
