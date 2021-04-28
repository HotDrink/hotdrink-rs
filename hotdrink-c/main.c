#include <stdio.h>
#include <string.h>
#include "bindings.h"

void handle_event(CEvent e) {
  const char *variable = e.variable;
  CEventData data = e.event_data;
  switch (e.event_type) {
    case Pending:
      printf("%s is pending\n", variable);
      break;
    case Ready:
      printf("%s = %i\n", variable, data);
      break;
    case Error:
      printf("%s failed: %s\n", variable, data);
      break;
    default: break;
  }
}

int main(void) {

  IntComponent *comp = component_new();
  component_subscribe(comp, "a", handle_event);
  component_subscribe(comp, "b", handle_event);
  component_subscribe(comp, "c", handle_event);
  component_set_variable(comp, "a", 3);
  component_update(comp);
  component_set_variable(comp, "b", 5);
  component_update(comp);
  component_free(comp);

  return 0;
}
