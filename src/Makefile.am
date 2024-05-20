bin_PROGRAMS = weather
weather_SOURCES = main.c
weather_CPPFLAGS = -I./include
weather_LDFLAGS = -pthread
weather_LDADD = $(METEO_LIB) -ldl -lgcc_s -lutil -lrt -lpthread -lm -lc


METEO_LIB = ./rust/target/debug/libmeteo.a

# Build the Rust library
all-local: $(METEO_LIB)

$(METEO_LIB):
	cd rust && cargo build
