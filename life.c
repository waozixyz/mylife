#include "raylib.h"
#include "raymath.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Placeholder for YAML parser
// You'll need to include and implement actual YAML parsing

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

void ReadConfig(const char* filename) {
    // Placeholder for YAML parsing
    // In a real implementation, you would use a YAML parser library here
    
    // For demonstration, we'll set some dummy data
    strcpy(config.name, "John Doe");
    strcpy(config.date_of_birth, "1990-01-01");
    config.life_expectancy = 80;
    
    config.period_count = 3;
    
    strcpy(config.life_periods[0].name, "Childhood");
    strcpy(config.life_periods[0].start, "1990-01-01");
    config.life_periods[0].color = BLUE;
    
    strcpy(config.life_periods[1].name, "Education");
    strcpy(config.life_periods[1].start, "2008-09-01");
    config.life_periods[1].color = GREEN;
    
    strcpy(config.life_periods[2].name, "Career");
    strcpy(config.life_periods[2].start, "2014-06-01");
    config.life_periods[2].color = RED;
}

char** GetYamlFiles(int* count) {
    // Placeholder for getting YAML files
    // In a real implementation, you would scan the "data" directory for .yaml and .yml files
    
    *count = 2;
    char** files = (char**)malloc(*count * sizeof(char*));
    files[0] = strdup("john.yaml");
    files[1] = strdup("jane.yaml");
    return files;
}

void DrawTimeline(int years) {
    int rows = (years + 3) / 4;
    float cellWidth = GetScreenWidth() / 48.0f;
    float cellHeight = (GetScreenHeight() - 100) / (rows + config.period_count); // Extra space for legend and UI
    
    time_t dob_time = 0;
    struct tm dob_tm = {0};
    sscanf(config.date_of_birth, "%d-%d-%d", &dob_tm.tm_year, &dob_tm.tm_mon, &dob_tm.tm_mday);
    dob_tm.tm_year -= 1900;
    dob_tm.tm_mon -= 1;
    dob_time = mktime(&dob_tm);
    
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < 48; j++) {
            time_t current_time = dob_time + (i * 48 + j) * 30 * 24 * 60 * 60; // Approximate month
            Color cellColor = WHITE;
            
            for (int k = 0; k < config.period_count; k++) {
                time_t period_start = 0;
                struct tm period_tm = {0};
                sscanf(config.life_periods[k].start, "%d-%d-%d", &period_tm.tm_year, &period_tm.tm_mon, &period_tm.tm_mday);
                period_tm.tm_year -= 1900;
                period_tm.tm_mon -= 1;
                period_start = mktime(&period_tm);
                
                time_t period_end = (k == config.period_count - 1) ? time(NULL) : mktime(&period_tm);
                if (k < config.period_count - 1) {
                    sscanf(config.life_periods[k+1].start, "%d-%d-%d", &period_tm.tm_year, &period_tm.tm_mon, &period_tm.tm_mday);
                    period_tm.tm_year -= 1900;
                    period_tm.tm_mon -= 1;
                    period_end = mktime(&period_tm);
                }
                
                if (current_time >= period_start && current_time < period_end) {
                    cellColor = config.life_periods[k].color;
                    break;
                }
            }
            
            DrawRectangle(j * cellWidth, i * cellHeight, cellWidth, cellHeight, cellColor);
        }
    }
    
    // Draw legend
    for (int i = 0; i < config.period_count; i++) {
        DrawRectangle(0, (rows + i) * cellHeight, GetScreenWidth(), cellHeight, config.life_periods[i].color);
        DrawText(TextFormat("%s (from %s)", config.life_periods[i].name, config.life_periods[i].start),
                 10, (rows + i) * cellHeight + 5, 20, BLACK);
    }
}

void DrawUI(void) {
    // Draw file selector
    DrawRectangleRec(fileSelector, LIGHTGRAY);
    DrawRectangleLinesEx(fileSelector, 1, BLACK);
    DrawText(selectedFileIndex >= 0 ? yamlFiles[selectedFileIndex] : "Select a file", fileSelector.x + 5, fileSelector.y + 5, 20, BLACK);
    
    // Draw life expectancy input
    DrawText("Life Expectancy:", 10, GetScreenHeight() - 40, 20, BLACK);
    DrawRectangle(160, GetScreenHeight() - 45, 50, 30, LIGHTGRAY);
    DrawText(lifeExpectancyInput, 165, GetScreenHeight() - 40, 20, BLACK);
    
    // Draw update button
    DrawRectangleRec(updateButton, GRAY);
    DrawText("Update", updateButton.x + 10, updateButton.y + 5, 20, BLACK);
}

void UpdateTimeline(void) {
    int years = atoi(lifeExpectancyInput);
    if (years < 1) years = 80; // Default value
    DrawTimeline(years);
}

int main(void) {
    InitWindow(800, 600, "My Life");
    SetTargetFPS(60);
    
    yamlFiles = GetYamlFiles(&yamlFileCount);
    
    fileSelector = (Rectangle){ 10, 10, 200, 30 };
    updateButton = (Rectangle){ GetScreenWidth() - 100, GetScreenHeight() - 45, 90, 30 };
    
    while (!WindowShouldClose()) {
        // Input handling
        if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
            Vector2 mousePoint = GetMousePosition();
            
            if (CheckCollisionPointRec(mousePoint, fileSelector)) {
                selectedFileIndex = (selectedFileIndex + 1) % yamlFileCount;
                ReadConfig(yamlFiles[selectedFileIndex]);
            }
            
            if (CheckCollisionPointRec(mousePoint, updateButton)) {
                UpdateTimeline();
            }
        }
        
        // Drawing
        BeginDrawing();
        ClearBackground(RAYWHITE);
        
        if (selectedFileIndex >= 0) {
            UpdateTimeline();
        }
        
        DrawUI();
        
        EndDrawing();
    }
    
    // Cleanup
    for (int i = 0; i < yamlFileCount; i++) {
        free(yamlFiles[i]);
    }
    free(yamlFiles);
    
    CloseWindow();
    return 0;
}