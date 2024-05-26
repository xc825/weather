# Telegram Weather Bot
Telegram weather bot

## Building steps

1. Clone this repository in Github
2. Create new Github codespace from repostitoy
3. clone and compile _telebot_
```
cd /workspaces && git clone "https://github.com/smartnode/telebot.git" && cd telebot && mkdir -p Build && cd Build && cmake ../ && make
```

4. Add Telagram bot token in file /workspaces/weather/token.txt
5. Compile Weather bot
```
./autogen.sh
./configure
make
```
6. Run Weather bot
```
cd /workspaces/weather
./src/weather;
````
7. Test Bot by sending messages to Telegram bot whos token is provided in /workspaces/weather/token.txt file.

8. To clean an rebuild run
```
make clean; make;
```