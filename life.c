#include "raylib.h"
#include "raymath.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <dirent.h>
#include <yaml.h>
#include <errno.h>

#define MAX_LIFE_PERIODS 20
#define MAX_FILENAME_LENGTH 256
#define MAX_FILES 50

typedef struct {
    char name[100];
    char start[11];
    Color color;
} LifePeriod;

typedef struct {
    char name[100];
    char date_of_birth[11];
    int life_expectancy;
    LifePeriod life_periods[MAX_LIFE_PERIODS];
    int period_count;
} Config;

Config config;
char** yamlFiles;
int yamlFileCount = 0;
int selectedFileIndex = -1;
char lifeExpectancyInput[4] = "80";
Rectangle updateButton;
Rectangle fileSelector;
bool dropdownActive = false;
void ReadConfig(const char* filename) {
    FILE* file = fopen(filename, "r");
    if (!file) {
        TraceLog(LOG_ERROR, "Failed to open file: %s (Error: %s)", filename, strerror(errno));
        return;
    }

    yaml_parser_t parser;
    yaml_event_t event;

    if (!yaml_parser_initialize(&parser)) {
        TraceLog(LOG_ERROR, "Failed to initialize YAML parser");
        fclose(file);
        return;
    }

    yaml_parser_set_input_file(&parser, file);

    int period_index = -1;
    char current_key[100] = "";
    bool in_life_periods = false;

    while (1) {
        if (!yaml_parser_parse(&parser, &event)) {
            TraceLog(LOG_ERROR, "Parser error: %s", parser.problem);
            break;
        }

        if (event.type == YAML_STREAM_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }

        switch (event.type) {
            case YAML_SCALAR_EVENT:
                if (strcmp((char*)event.data.scalar.value, "life_periods") == 0) {
                    in_life_periods = true;
                } else if (in_life_periods) {
                    if (strcmp(current_key, "name") == 0) {
                        strncpy(config.life_periods[period_index].name, (char*)event.data.scalar.value, sizeof(config.life_periods[period_index].name) - 1);
                    } else if (strcmp(current_key, "start") == 0) {
                        strncpy(config.life_periods[period_index].start, (char*)event.data.scalar.value, sizeof(config.life_periods[period_index].start) - 1);
                    } else if (strcmp(current_key, "color") == 0) {
                        unsigned int color;
                        sscanf((char*)event.data.scalar.value, "#%x", &color);
                        config.life_periods[period_index].color = (Color){
                            (color >> 16) & 0xFF,
                            (color >> 8) & 0xFF,
                            color & 0xFF,
                            255
                        };
                    }
                } else {
                    if (strcmp(current_key, "name") == 0) {
                        strncpy(config.name, (char*)event.data.scalar.value, sizeof(config.name) - 1);
                    } else if (strcmp(current_key, "date_of_birth") == 0) {
                        strncpy(config.date_of_birth, (char*)event.data.scalar.value, sizeof(config.date_of_birth) - 1);
                    } else if (strcmp(current_key, "life_expectancy") == 0) {
                        config.life_expectancy = atoi((char*)event.data.scalar.value);
                    }
                }
                strncpy(current_key, (char*)event.data.scalar.value, sizeof(current_key) - 1);
                break;
            case YAML_SEQUENCE_START_EVENT:
                if (in_life_periods) {
                    period_index = -1;
                }
                break;
            case YAML_MAPPING_START_EVENT:
                if (in_life_periods) {
                    period_index++;
                }
                break;
            default:
                break;
        }

        yaml_event_delete(&event);
    }

    yaml_parser_delete(&parser);
    fclose(file);

    config.period_count = period_index + 1;
    TraceLog(LOG_INFO, "Loaded %d life periods\n", config.period_count);
}


char** GetYamlFiles(int* count) {
    DIR* dir;
    struct dirent* ent;
    char** files = NULL;
    int file_count = 0;

    dir = opendir("data");
    if (dir != NULL) {
        while ((ent = readdir(dir)) != NULL) {
            if (strstr(ent->d_name, ".yaml") || strstr(ent->d_name, ".yml")) {
                files = realloc(files, (file_count + 1) * sizeof(char*));
                char fullPath[256];
                snprintf(fullPath, sizeof(fullPath), "data/%s", ent->d_name);
                files[file_count] = strdup(fullPath);
                TraceLog(LOG_INFO, "Found YAML file: %s", fullPath);
                file_count++;
            }
        }
        closedir(dir);
    } else {
        TraceLog(LOG_ERROR, "Failed to open 'data' directory");
    }

    *count = file_count;
    return files;
}
void DrawLegend(void) {
    float legendHeight = 30; // Height for each legend item
    float legendY = GetScreenHeight() - 50 - (config.period_count * legendHeight);

    for (int i = 0; i < config.period_count; i++) {
        DrawRectangle(0, legendY + (i * legendHeight), GetScreenWidth(), legendHeight, config.life_periods[i].color);
        DrawText(TextFormat("%s (from %s)", config.life_periods[i].name, config.life_periods[i].start),
                 10, legendY + (i * legendHeight) + 5, 20, BLACK);
    }
}


void DrawTimeline(int years) {
    int rows = (years + 3) / 4;
    float cellSize = GetScreenWidth() / 48.0f;

    struct tm dob_tm = {0};
    sscanf(config.date_of_birth, "%d-%d-%d", &dob_tm.tm_year, &dob_tm.tm_mon, &dob_tm.tm_mday);
    dob_tm.tm_year -= 1900;
    dob_tm.tm_mon -= 1;
    time_t dob_time = mktime(&dob_tm);

    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < 48; j++) {
            time_t current_time = dob_time + (i * 48 + j) * 30 * 24 * 60 * 60;
            Color cellColor = WHITE;

            for (int k = 0; k < config.period_count; k++) {
                struct tm period_tm = {0};
                sscanf(config.life_periods[k].start, "%d-%d-%d", &period_tm.tm_year, &period_tm.tm_mon, &period_tm.tm_mday);
                period_tm.tm_year -= 1900;
                period_tm.tm_mon -= 1;
                time_t period_start = mktime(&period_tm);

                time_t period_end = (k == config.period_count - 1) ? time(NULL) : mktime(&period_tm);
                if (k < config.period_count - 1) {
                    sscanf(config.life_periods[k + 1].start, "%d-%d-%d", &period_tm.tm_year, &period_tm.tm_mon, &period_tm.tm_mday);
                    period_tm.tm_year -= 1900;
                    period_tm.tm_mon -= 1;
                    period_end = mktime(&period_tm);
                }

                if (current_time >= period_start && current_time < period_end) {
                    cellColor = config.life_periods[k].color;
                    break;
                }
            }

            DrawRectangle(j * cellSize, i * cellSize, cellSize, cellSize, cellColor);
            DrawRectangleLines(j * cellSize, i * cellSize, cellSize, cellSize, BLACK); // Drawing the border
        }
    }
}

void DrawUI(void) {
    DrawRectangleRec(fileSelector, LIGHTGRAY);
    DrawRectangleLinesEx(fileSelector, 1, BLACK);
    DrawText(selectedFileIndex >= 0 ? yamlFiles[selectedFileIndex] : "Select a file", fileSelector.x + 5, fileSelector.y + 5, 20, BLACK);

    if (dropdownActive) {
        for (int i = 0; i < yamlFileCount; i++) {
            Rectangle itemRect = {fileSelector.x, fileSelector.y + (i + 1) * 30, fileSelector.width, 30};
            DrawRectangleRec(itemRect, WHITE);
            DrawRectangleLinesEx(itemRect, 1, BLACK);
            DrawText(yamlFiles[i], itemRect.x + 5, itemRect.y + 5, 20, BLACK);
        }
    }

    DrawLegend();
    
    DrawText("Life Expectancy:", 10, GetScreenHeight() - 40, 20, BLACK);
    DrawRectangle(190, GetScreenHeight() - 45, 50, 30, LIGHTGRAY);
    DrawText(lifeExpectancyInput, 200, GetScreenHeight() - 40, 20, BLACK);

    //DrawRectangleRec(updateButton, GRAY);
    //DrawText("Update", updateButton.x + 10, updateButton.y + 5, 20, BLACK);
}


void UpdateLifeExpectancy(void) {
    int years = atoi(lifeExpectancyInput);
    if (years > 0) {
        config.life_expectancy = years;
    }
}

void UpdateTimeline(void) {
    UpdateLifeExpectancy();
    DrawTimeline(config.life_expectancy);
}

int main(void) {
    DIR* dir = opendir("data");
    if (dir) {
        closedir(dir);
    } else {
        TraceLog(LOG_ERROR, "The 'data' directory does not exist or is not accessible");
        return 1;
    }

    InitWindow(800, 600, "My Life");
    SetTargetFPS(60);

    yamlFiles = GetYamlFiles(&yamlFileCount);

    fileSelector = (Rectangle){ 10, 10, 200, 30 };
    updateButton = (Rectangle){ GetScreenWidth() - 100, GetScreenHeight() - 45, 90, 30 };

    while (!WindowShouldClose()) {
        if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
            Vector2 mousePoint = GetMousePosition();

            if (CheckCollisionPointRec(mousePoint, fileSelector)) {
                dropdownActive = !dropdownActive;
            } else if (dropdownActive) {
                for (int i = 0; i < yamlFileCount; i++) {
                    Rectangle itemRect = {fileSelector.x, fileSelector.y + (i + 1) * 30, fileSelector.width, 30};
                    if (CheckCollisionPointRec(mousePoint, itemRect)) {
                        selectedFileIndex = i;
                        dropdownActive = false;
                        ReadConfig(yamlFiles[selectedFileIndex]);
                        break;
                    }
                }
            }

            if (CheckCollisionPointRec(mousePoint, updateButton)) {
                UpdateTimeline();
            }
        }

        int key = GetKeyPressed();
        if (key >= 48 && key <= 57) { // Numbers 0-9
            int len = strlen(lifeExpectancyInput);
            if (len < 3) {
                lifeExpectancyInput[len] = (char)key;
                lifeExpectancyInput[len + 1] = '\0';
            }
        } else if (key == KEY_BACKSPACE) {
            int len = strlen(lifeExpectancyInput);
            if (len > 0) {
                lifeExpectancyInput[len - 1] = '\0';
            }
        }


        BeginDrawing();
        ClearBackground(RAYWHITE);

        if (selectedFileIndex >= 0) {
            UpdateTimeline();
        }

        DrawUI();

        EndDrawing();
    }

    for (int i = 0; i < yamlFileCount; i++) {
        free(yamlFiles[i]);
    }
    free(yamlFiles);

    CloseWindow();
    return 0;
}