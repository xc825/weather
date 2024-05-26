bin_PROGRAMS = weather
weather_SOURCES = main.c
weather_CPPFLAGS = -I./include -I./../../telebot/include 
#	-I/usr/include/apr-1.0
weather_LDFLAGS = -pthread -L./../../telebot/Build
weather_LDADD = $(METEO_LIB) -ltelebot_static -ldl -lgcc_s -lutil -lrt\
				-lpthread -lm -lc -lssl -lcrypto -ljson-c -lcurl
#  $(TELEBOT_STATIC_LIB)


METEO_LIB = ./rust/target/debug/libmeteo.a
#TELEBOT_STATIC_LIB = ./../../telebot/Build/libtelebot-static.a

# Build the Rust library
all-local: $(METEO_LIB)

$(METEO_LIB):
	cd rust && cargo build

clean-local:
	rm -f ./rust/target/debug/libmeteo.a

#cd rust && cargo clean
