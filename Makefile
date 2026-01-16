VERILATOR = verilator
VERILATOR_FLAGS = --cc --Mdir ./obj -Wall

SV_SRC = $(wildcard ./src/*.sv)
OBJ = $(patsubst ./src/%.sv, ./obj/%.o, $(SV_SRC))

all: $(OBJ)

./obj/%.o: ./src/%.sv
	$(VERILATOR) $(VERILATOR_FLAGS) $<

clean:
	rm -rf obj/

.PHONY: all clean
