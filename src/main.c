#include <stdio.h>
#include "meteo.h"
#include <stdlib.h>
#include <pthread.h>
#include <string.h>
#include <unistd.h>
#include <telebot.h>

#define SIZE_OF_ARRAY(array) (sizeof(array) / sizeof(array[0]))

int read_token(char *file_name, char *token_buffer, size_t token_buffer_size);
int bot_init(char *token, telebot_handler_t *handle, telebot_user_t *me);
int bot_run(telebot_handler_t handle);

int main()
{
    char token[1024];
    char token_file_path[1024] = "token.txt";
    telebot_handler_t handle;
    telebot_user_t me;

    if (read_token(token_file_path, token, SIZE_OF_ARRAY(token)) != 0)
    {
        printf("Failed to read token\n");
        return -1;
    }

    if (bot_init(token, &handle, &me) != 0)
    {
        printf("Failed to init bot\n");
        return -1;
    }

    bot_run(handle);
    telebot_destroy(handle);

    return 0;
}

int read_token(char *file_name, char *token_buffer, size_t token_buffer_size)
{
    FILE *fp = fopen(file_name, "r");
    if (fp == NULL)
    {
        printf("Failed to open [%s] file\n", file_name);
        return -1;
    }

    if (fgets(token_buffer, token_buffer_size, fp) == NULL)
    {
        printf("Failed to read token\n");
        fclose(fp);
        return -1;
    }
    fclose(fp);
}

int bot_init(char *token, telebot_handler_t *handle, telebot_user_t *me)
{
    if (telebot_create(handle, token) != TELEBOT_ERROR_NONE)
    {
        printf("Telebot create failed\n");
        return -1;
    }

    if (telebot_get_me(*handle, me) != TELEBOT_ERROR_NONE)
    {
        printf("Failed to get bot information\n");
        telebot_destroy(*handle);
        return -1;
    }

    printf("ID: %d\n", me->id);
    printf("First Name: %s\n", me->first_name);
    printf("User Name: %s\n", me->username);

    telebot_put_me(me);
    return 0;
}

int bot_run(telebot_handler_t handle)
{
    char weather_buffer[1024];
    int index, count, offset = -1;
    telebot_error_e ret;
    telebot_message_t message;
    telebot_update_type_e update_types[] = {TELEBOT_UPDATE_TYPE_MESSAGE};

    while (1)
    {
        telebot_update_t *updates;
        ret = telebot_get_updates(handle, offset, 20, 0, update_types, 0, &updates, &count);
        if (ret != TELEBOT_ERROR_NONE)
            continue;
        printf("Number of updates: %d\n", count);
        for (index = 0; index < count; index++)
        {
            message = updates[index].message;
            if (message.text)
            {
                printf("%s: %s \n", message.from->first_name, message.text);
                if (strstr(message.text, "/dice"))
                {
                    telebot_send_dice(handle, message.chat->id, false, 0, "");
                }
                else
                {
                    char str[4096];
                    if (strstr(message.text, "/start"))
                    {
                        snprintf(str, SIZE_OF_ARRAY(str),
                            "Hello %s!\nThis is weather bot.\n"
                            "Send city name or location coordinates to get weather information\n"
                            "Example:\n"
                            "<b>Riga</b>\nor\n<b>56.9,24.1</b>\n",
                            message.from->first_name);
                    }
                    else
                    {
                        weather(message.text, weather_buffer, SIZE_OF_ARRAY(weather_buffer));
                        snprintf(str, SIZE_OF_ARRAY(str), "<i>%s</i>", weather_buffer);
                    }
                    ret = telebot_send_message(handle, message.chat->id, str, "HTML", false, false, updates[index].message.message_id, "");
                }
                if (ret != TELEBOT_ERROR_NONE)
                {
                    printf("Failed to send message: %d \n", ret);
                }
            }
            offset = updates[index].update_id + 1;
        }
        telebot_put_updates(updates, count);
        sleep(3);
    }

    return ret;
}

