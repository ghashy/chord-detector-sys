#include <bridge.h>
#include <stdio.h>

C_Chromagram *c_init(int frameSize, int sampleRate) {
  return init(frameSize, sampleRate);
}
void c_destroy(C_Chromagram* c) {
  destroy(c);
}
void c_process_audio_frame(C_Chromagram* c, double* f) {
  process_audio_frame(c, f);
}
int c_is_ready(C_Chromagram* c) {
  return is_ready(c);
} 
void sayhello() {
  printf("Hello from C lang!\n");
}
