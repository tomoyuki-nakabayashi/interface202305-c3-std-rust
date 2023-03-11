#include <my_component.h>
#include <esp_log.h>

static const char *TAG = "my_component";

void hello_from_c(void)
{
    ESP_LOGI(TAG, "hello!");
}
