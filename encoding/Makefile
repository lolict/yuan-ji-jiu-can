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

# SMRL engine
SMRL_OBJS = smrl_engine.o
TEST_SMRL = test_smrl

# Hierarchical carry (层级进位制)
HIER_CARRY = hierarchical_carry

# Full system demos
TEST_FULL_V1 = test_full_system
TEST_FULL_V2 = test_full_system_v2
TEST_LIFECYCLE = test_lifecycle
TEST_SWARM = test_swarm
TEST_SELFBOOT = test_selfboot
TEST_CHRYSALIS = test_chrysalis
TEST_MLIL3D = test_mlil3d
TEST_HOLO = test_holo
TEST_MIND_ORGAN = test_mind_organ
MAIN = hunyuanos

# Phonetic codec (z-api.txt + symbols_list.txt)
PHONO = phono
PHONO_SRC = phono_codec.c

HIER_CARRY = hierarchical_carry
HIER_CARRY_SRC = hierarchical_carry.c

ALL = $(TEST_COLOR) $(TEST_LEVEL) $(TEST_TYPE) $(TEST_DIFF) $(TEST_WOV) \
      $(TEST_FULL_V1) $(TEST_FULL_V2) $(TEST_LIFECYCLE) $(MAIN) $(TEST_SWARM) \
      $(TEST_SELFBOOT) $(TEST_CHRYSALIS) $(TEST_MLIL3D) $(TEST_HOLO) \
      $(TEST_MIND_ORGAN) $(TEST_ONT_BRIDGE) $(TEST_SMRL) $(TEST_SYNC) $(PHONO) $(HIER_CARRY)

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

$(MAIN): main.o $(CHAIN_OBJS) $(MEMORY_OBJS) lifecycle_engine.o smrl_engine.o sync_engine.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

main.o: main.c chain_block.h bootstrap_chain.h cloud_observer.h chain_persist.h memory_chain.h growth_ring.h version_graph.h observer_v2.h lifecycle_engine.h smrl_engine.h sync_engine.h
	$(CC) $(CFLAGS) -c $< -o $@

$(TEST_SWARM): test_swarm.o njsg_swarm.o swarm_agents.o chain_block.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_SELFBOOT): test_selfboot.o bootstrap_compiler.o chain_block.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_CHRYSALIS): test_chrysalis.o chrysalis_shell.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

$(TEST_MLIL3D): test_mlil3d.o mlil_visual.o chain_block.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS) -lm

$(TEST_HOLO): test_holo.o holo_viewer.o mlil_visual.o njsg_swarm.o swarm_agents.o chain_block.o memory_chain.o growth_ring.o version_graph.o observer_v2.o lifecycle_engine.o cloud_observer.o chain_persist.o bootstrap_chain.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS) -lm

$(TEST_MIND_ORGAN): test_mind_organ.o mind_organ.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS) -lm

$(TEST_SMRL): test_smrl.o smrl_engine.o chain_block.o
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

smrl_engine.o: smrl_engine.c smrl_engine.h chain_block.h
	$(CC) $(CFLAGS) -c $< -o $@

# Sync engine (override test_ pattern to avoid builtin rule conflict)
TEST_SYNC_OBJS = test_sync.o sync_engine.o smrl_engine.o chain_block.o
test_sync: $(TEST_SYNC_OBJS)
	$(CC) $(CFLAGS) $(TEST_SYNC_OBJS) -o $@ $(LDFLAGS)

sync_engine.o: sync_engine.c sync_engine.h smrl_engine.h chain_block.h
	$(CC) $(CFLAGS) -c $< -o $@

# Container ontology bridge (all 6 sub-containers integrated)
ONT_BRIDGE_OBJS = container_ontology_bridge.o mind_organ.o global_control.o property_judge.o radix_star.o encoding_bridge_emit.o type_discipline.o hanzi_table.o mccp_encdec.o container_pipeline.o
encoding_bridge_emit.o: encoding_bridge_emit.c encoding_bridge.h
	$(CC) $(CFLAGS) -c $< -o $@

TEST_ONT_BRIDGE = test_ontology_bridge
$(TEST_ONT_BRIDGE): test_ontology_bridge.o $(ONT_BRIDGE_OBJS)
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
	@echo "--- Mind Organ (距离感知聚类) ---"
	@./$(TEST_MIND_ORGAN) || true
	@echo ""
	@echo "--- Ontology Bridge (6子容器集成) ---"
	@./$(TEST_ONT_BRIDGE) || true
	@echo ""

$(PHONO): $(PHONO_SRC)
	$(CC) $(CFLAGS) -o $@ $<

$(HIER_CARRY): $(HIER_CARRY_SRC)
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f $(ALL) *.o

lib: $(COLOR_OBJS)
	@echo "Color system library: $(COLOR_OBJS)"
