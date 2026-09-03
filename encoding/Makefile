CC = cc
CFLAGS = -Wall -Wextra -std=c99 -O2
LDFLAGS = -lm

COLOR_SRCS = color_system.c type_discipline.c diffraction_engine.c weaving_balancer.c
COLOR_OBJS = $(COLOR_SRCS:.c=.o)

TEST_COLOR = test_color_system
TEST_LEVEL = test_level_stack
TEST_TYPE  = test_type_discipline
TEST_DIFF  = test_diffraction
TEST_WOV   = test_weaving

# Chain subsystem
CHAIN_SRCS = chain_block.c cloud_observer.c chain_persist.c bootstrap_chain.c
CHAIN_OBJS = $(CHAIN_SRCS:.c=.o)

# Memory/Growth/Graph subsystem
MEMORY_SRCS = memory_chain.c growth_ring.c version_graph.c observer_v2.c
MEMORY_OBJS = $(MEMORY_SRCS:.c=.o)

# Lifecycle engine
LIFE_OBJS = lifecycle_engine.c

# Full system demos
TEST_FULL_V1 = test_full_system
TEST_FULL_V2 = test_full_system_v2
TEST_LIFECYCLE = test_lifecycle
TEST_SWARM = test_swarm
TEST_SELFBOOT = test_selfboot
TEST_CHRYSALIS = test_chrysalis
TEST_MLIL3D = test_mlil3d
MAIN = hunyuanos

ALL = $(TEST_COLOR) $(TEST_LEVEL) $(TEST_TYPE) $(TEST_DIFF) $(TEST_WOV) \
      $(TEST_FULL_V1) $(TEST_FULL_V2) $(TEST_LIFECYCLE) $(MAIN) $(TEST_SWARM) \
      $(TEST_SELFBOOT) $(TEST_CHRYSALIS) $(TEST_MLIL3D)

.PHONY: all clean test lib

all: $(ALL)

color_system.o: color_system.c color_system.h type_discipline.h diffraction_engine.h weaving_balancer.h
	$(CC) $(CFLAGS) -c $< -o $@

type_discipline.o: type_discipline.c type_discipline.h
	$(CC) $(CFLAGS) -c $< -o $@

diffraction_engine.o: diffraction_engine.c diffraction_engine.h type_discipline.h
	$(CC) $(CFLAGS) -c $< -o $@

weaving_balancer.o: weaving_balancer.c weaving_balancer.h
	$(CC) $(CFLAGS) -c $< -o $@

$(TEST_COLOR): test_color_system.o $(COLOR_OBJS)
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_LEVEL): test_level_stack.o type_discipline.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_TYPE): test_type_discipline.o type_discipline.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_DIFF): test_diffraction.o type_discipline.o diffraction_engine.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_WOV): test_weaving.o weaving_balancer.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

test_%.o: test_%.c
	$(CC) $(CFLAGS) -c $< -o $@

$(TEST_FULL_V1): test_full_system.o $(CHAIN_OBJS)
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_FULL_V2): test_full_system_v2.o $(CHAIN_OBJS) $(MEMORY_OBJS)
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_LIFECYCLE): test_lifecycle.o $(CHAIN_OBJS) $(MEMORY_OBJS) lifecycle_engine.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(MAIN): main.o $(CHAIN_OBJS) $(MEMORY_OBJS) lifecycle_engine.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_SWARM): test_swarm.o njsg_swarm.o swarm_agents.o chain_block.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_SELFBOOT): test_selfboot.o bootstrap_compiler.o chain_block.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_CHRYSALIS): test_chrysalis.o chrysalis_shell.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_MLIL3D): test_mlil3d.o mlil_visual.o chain_block.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS) -lm

test: all
	@echo ""
	@echo "====== Running All Tests ====="
	@echo ""
	@echo "--- Type Discipline ---"
	@./$(TEST_TYPE) || true
	@echo ""
	@echo "--- Level Stack ---"
	@./$(TEST_LEVEL) || true
	@echo ""
	@echo "--- Diffraction Engine ---"
	@./$(TEST_DIFF) || true
	@echo ""
	@echo "--- Weaving Balancer ---"
	@./$(TEST_WOV) || true
	@echo ""
	@echo "--- Color System Full ---"
	@./$(TEST_COLOR) || true
	@echo ""

clean:
	rm -f $(ALL) *.o

lib: $(COLOR_OBJS)
	@echo "Color system library: $(COLOR_OBJS)"
